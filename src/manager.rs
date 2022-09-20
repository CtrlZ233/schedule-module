use alloc::collections::btree_map::BTreeMap;
use crate::scheduler::{Scheduler, TaskId};
use crate::task_container::TaskContainer;
use spin::Mutex;
use alloc::sync::Arc;

pub struct Manager<T, S> where S: Scheduler {
    tasks: Mutex<BTreeMap<usize, Arc<T>>>,
    pub scheduler: Arc<Mutex<Box<S>>>,
    prio_map: Mutex<BTreeMap<usize, usize>>,
    current: Mutex<Option<usize>>,
}

impl<T, S: Scheduler> Manager<T, S> {
    /// 新建 Manager
    pub fn new(scheduler: S) -> Self {
        Manager {
            tasks: Mutex::new(BTreeMap::new()),
            scheduler: Arc::new(Mutex::new(Box::new(scheduler))),
            prio_map: Mutex::new(BTreeMap::new()),
            current: Mutex::new(None),
        }
    }

    pub fn fetch(&self) -> Option<Arc<T>> {
        let task_id = self.scheduler.lock().pop();
        if let Some(tid) = task_id {
            *self.current.lock() = Some(tid.tid);
            return self.get(tid.tid);
        }
        None
    }

    pub fn get(&self, tid: usize) -> Option<Arc<T>> {
        if let Some(ret) = self.tasks.lock().get(&tid) {
            return Some(ret.clone());
        }
        None
    }

    pub fn re_back(&self, tid: usize) -> bool {
        let res = self.scheduler.try_lock();
        let prio_map = self.prio_map.try_lock();
        if res.is_none() {
            return false;
        }
        if prio_map.is_none() {
            return false;
        }
        let op_prio = prio_map.unwrap().get(&tid).copied();
        match op_prio {
            Some(prio) => res.unwrap().push(TaskId{tid, prio}),
            _ => return false,
        }
        true
    }

    pub fn add(&self, task: Arc<T>, tid: usize, prio: usize) {
        self.scheduler.lock().push(TaskId{tid, prio});
        self.tasks.lock().insert(tid, task);
        self.prio_map.lock().insert(tid, prio);
    }

    pub fn remove(&self, tid: usize) {
        self.tasks.lock().remove(&tid);
        self.prio_map.lock().remove(&tid);
    }

    /// 判断 Manager 中是否有任务
    pub fn empty(&self) -> bool { self.scheduler.lock().empty() }
}

