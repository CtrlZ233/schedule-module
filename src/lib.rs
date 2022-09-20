extern crate alloc;


mod manager;
mod scheduler;

#[derive(Debug, Clone)]
struct Task {
    tid: usize,
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::manager::Manager;
    use crate::scheduler::PrioScheduler;
    use crate::Task;
    use crate::task_container::VecContainer;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn manager_test() {
        let scheduler = PrioScheduler::new();
        let mut manager: Manager<Task, PrioScheduler> = Manager::new(scheduler);
        manager.add(Arc::new(Task{tid: 0}), 0, 1);
        manager.add(Arc::new(Task{tid: 1}), 1, 4);
        manager.add(Arc::new(Task{tid: 2}), 2, 2);
        manager.add(Arc::new(Task{tid: 3}), 3, 3);
        let tid = manager.fetch().unwrap().tid;
        let task = manager.get(tid).unwrap();
        println!("{:?}", task);
    }

}