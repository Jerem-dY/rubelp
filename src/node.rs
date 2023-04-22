use pyo3::prelude::*;

#[pyclass(module = "mbr", subclass, text_signature="(txt: str, i: int, lvl: int)")]
#[derive(Clone)]
#[derive(Debug)]
pub struct Node{
    #[pyo3(get)]
    pub children: Vec<usize>,
    #[pyo3(get)]
    pub data: String,
    #[pyo3(get)]
    pub index: usize,
    #[pyo3(get)]
    pub level: usize
}

#[pymethods]
impl Node{

    #[new]
    pub fn new(txt: String, lvl: usize, i: usize) -> Self{
        Node { 
            data: txt, 
            index: i, 
            level: lvl, 
            children: Vec::<usize>::new()
        }
    }

}


impl Node{

    pub fn add_child(&mut self, child: &Node) -> Result<()>{

        self.children.push(child.index);

        Ok(())
    }

}
