use std::{cell::RefCell, rc::Rc};

use crate::{IterationVar, Task, ThrillerResult, Var};

/// Schedule tree.
pub struct ScheduleTree {
    root: Rc<RefCell<ScheduleNode>>,
}

impl ScheduleTree {
    /// Create a new [`ScheduleTree`] with the given root node.
    pub fn new(root: Rc<RefCell<ScheduleNode>>) -> Self {
        ScheduleTree { root }
    }
}

/// Schedule node.
pub struct ScheduleNode {
    pub(crate) data: SchduleData,
    pub(crate) childrens: Vec<Rc<RefCell<ScheduleNode>>>,
    pub(crate) op: Vec<Box<dyn Task>>,
}

impl ScheduleNode {
    /// Create a new [`ScheduleNode`] with the given iteration variables.
    pub fn new(ivar: Vec<Rc<IterationVar>>) -> Self {
        ScheduleNode {
            data: SchduleData { loop_info: ivar },
            childrens: Vec::new(),
            op: vec![],
        }
    }

    /// Add a child to the node.
    pub fn add_child(&mut self, child: Rc<RefCell<ScheduleNode>>) {
        self.childrens.push(child);
    }

    /// Mount a task to the node.
    pub fn mount_task(&mut self, task: Box<dyn Task>) {
        self.op.push(task);
    }

    fn emit_loop(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        let mut indent = 0;

        for ivar in self.data.loop_info.iter() {
            let (start, end) = ivar.get_domain();
            let ivar_name = ivar.get_name();

            code.push_str(&format!(
                "{indent}for(int {var} = {start}, {var} < {end}; {var}++){{\n",
                indent = " ".repeat(indent),
                var = ivar_name,
                start = start,
                end = end
            ));

            indent += 4;
        }

        Ok(code)
    }
}

/// Schedule data.
pub(crate) struct SchduleData {
    pub(crate) loop_info: Vec<Rc<IterationVar>>,
}

impl Task for ScheduleTree {
    fn emit(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        let root = self.root.borrow();

        code += &root.emit_loop()?;

        // Ok(code)

        todo!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }
}

impl Task for ScheduleNode {
    fn emit(&self) -> ThrillerResult<String> {
        let mut code = String::new();

        for op in self.op.iter() {
            code += &op.emit()?;
        }

        Ok(code)
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }
}
