use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
pub struct TaskId {
    pub tid: usize,
    pub prio: usize,
}

impl Eq for TaskId {

}

impl PartialEq<Self> for TaskId {
    fn eq(&self, other: &Self) -> bool {
        self.prio == other.prio
    }
}

impl PartialOrd<Self> for TaskId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.prio.partial_cmp(&other.prio)
    }
}

impl Ord for TaskId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub trait Scheduler: 'static {
    // 如果 tid 不存在，表明将一个新线程加入线程调度
    // 否则表明一个已有的线程要继续运行
    fn push(&mut self, tid: TaskId);

    // 从若干可运行线程中选择一个运行
    fn pop(&mut self) -> Option<TaskId>;

    fn empty(&self) -> bool;
}

pub struct PrioScheduler {
    prio_queue: BinaryHeap<TaskId>,
}

impl PrioScheduler {
    pub fn new() -> PrioScheduler {
        PrioScheduler {
            prio_queue: BinaryHeap::new()
        }
    }
}

impl Scheduler for PrioScheduler {
    fn push(&mut self, tid: TaskId) {
        self.prio_queue.push(tid);
    }

    fn pop(&mut self) -> Option<TaskId> {
        self.prio_queue.pop()
    }

    fn empty(&self) -> bool {
        self.prio_queue.is_empty()
    }
}