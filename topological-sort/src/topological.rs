#[allow(dead_code)]
#[allow(unused_variables)]
use crate::{cell_id::CellId, expr::Expr};
use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
};
//Interface
//Return dependecies
//resolve
//Nodes with no dependecies

/// A directed graph is represented as a hash map mapping a vertex `a`
/// to a hash set of the vertices connected to it with an edge starting at `a`.
///
/// For example, graph `a -> b, b -> c, a -> c` will be represented as:
///
/// HashMap{
///   a: HashSet{ b, c },
///   b: HashSet{ c }
/// }
type Graph<T>=HashMap<t,HashSet<T>>;
pub struct State<T>{
   depends_on:Graph<T>,
   dependents:Graph<T>,
   no_deps:Vect<T>,
}

//depends_on
//'axum'->{'axum-core', 'mime'}
//'axum-core->{'mime', 'tower-layer'}

//depends
//'axum-core'->{'axum'}
//'mime'->{'axum', 'axum-core'}
//'tower-layer'->{'axum' 'axum-core'}

//Build graphs
#[inline]
pub fn add_edge<T>(graph: &mut Graph<T>, from: T, to: T)
where
  T: Eq + Hash + Copy,
{
  graph
    .entry(from)
    .and_modify(|pointees| {
      pointees.insert(to);
    })
    .or_insert_with(|| {
      let mut s = HashSet::new();
      s.insert(to);
      s
    });
}

//State track
impl<T> State<T>
where
  T: Eq + std::hash::Hash,
{
  pub fn get_dependents(self: &Self, dependency: &T) -> Option<&HashSet<T>> {
    // it's possible to replace the return type with HashSet<T>, but then we'll need to allocate
    self.dependents.get(dependency)
  }

  pub fn is_resolved(self: &Self) -> bool {
    self.depends_on.is_empty()
  }
}
/// Preprocessed state for Kahn's topological sorting algorithm.
///
/// Allows (expected) O(1) dependencies & dependents retrieval for any `node_id: T`
/// and stores `no_deps` vector.
impl<T> State<T>
where
  T: Copy + Eq + std::hash::Hash,
{
  pub fn resolve(self: &mut Self, dependent: &T, dependency: &T) {
    if let Some(dependencies) = self.depends_on.get_mut(dependent) {
      dependencies.remove(&dependency);

      if dependencies.is_empty() {
        self.no_deps.push(*dependent);

        // to be able to report unresolved
        self.depends_on.remove(dependent);
      }
    }
  }

  pub fn unresolved(self: &Self) -> impl Iterator<Item = &T> {
   self.depends_on.keys()
 }
}

/// Performs topological sorting for a `T` that can be converted to `State<Id>`
/// (`From<T>` is implemented for `State<Id>`).
///
/// ## Implementation Notes
///
/// The following code in this while loop is possible to replace with
/// the following line, but we prefer significantly better readability over
/// slightly better performance (this avoids one clone):
/// `state.resolve_for_dependants_of(&cell_id);`
pub fn topological_sort<T, Id>(deps: T) -> Result<Vec<Id>, Box<dyn std::error::Error>>
where
  Id: Eq + std::hash::Hash + Copy + std::fmt::Debug,
  State<Id>: From<T>,
{
  let mut res = vec![];
  let mut state = State::from(deps);

  while let Some(cell_id) = state.no_deps.pop() {
    res.push(cell_id);

    if let Some(dependents) = state.get_dependents(&cell_id) {
      for dependent in dependents.clone() {
        state.resolve(&dependent, &cell_id);
      }
    }
  }

  if !state.is_resolved() {
    return Err(
      format!(
        "cycle or non-computable cell reference detected in cells: {:?}",
        state.unresolved().collect::<Vec<_>>()
      )
      .into(),
    );
  }

  Ok(res)
}

//Topological order from a spreedsheet data 
impl From<&HashMap<CellId, Expr>> for State<CellId> {
   fn from(exprs: &HashMap<CellId, Expr>) -> State<CellId> {
     let mut graphs = State::default();
 
     for (&cell_id, expr) in exprs.iter() {
       let dependencies = expr.get_deps();
 
       if dependencies.is_empty() {
         graphs.no_deps.push(cell_id);
       } else {
         for dependency_cell_id in dependencies {
           add_edge(&mut graphs.depends_on, cell_id, dependency_cell_id);
           add_edge(&mut graphs.dependents, dependency_cell_id, cell_id);
         }
       }
     }
 
     graphs
   }
 }
pub fn eval (exprs:&HashMap<CellId,Expr>)->Result<HashMap<CellId,Expr>,Box<dyn Error>>{
  let mut values=HashMap::new;
  for cell_id in topological_sort(expres)?{
    if let Some(expr)=exprs.get(&cell_id) {
    match expr{
      Expr::Num(n)=>{
        values.insert(cell_id, *n);
      }
      Expr::CellRef (another_cell_id)=>{
        if let Some (another_value)=values.get(another_cell_id){
          values.insert(cell_id, *another_value)
        }
      }
      Exper::Apply {..}=>
      let value=expr.eval(&values)?;
      values.insert(cell_id,value)
    }
  }
}
OK(values)}


