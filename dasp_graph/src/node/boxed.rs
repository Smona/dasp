use crate::{Buffer, Input, Node};
use core::fmt;
use core::ops::{Deref, DerefMut};

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
pub struct BoxedNode<N: Node + ?Sized = dyn Node>(pub Box<N>);

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
///
/// Useful when the ability to send nodes from one thread to another is required. E.g. this is
/// common when initialising nodes or the audio graph itself on one thread before sending them to
/// the audio thread.
pub struct BoxedNodeSend<N: Node + ?Sized + Send = dyn Node + Send>(pub Box<N>);

impl<T: Node + 'static> BoxedNode<T> {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new(node: T) -> Self {
        Self::from(Box::new(node))
    }
}

impl<T: Node + Send + 'static> BoxedNodeSend<T> {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new(node: T) -> Self {
        Self::from(Box::new(node))
    }
}

impl<T: Node> Node for BoxedNode<T> {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        self.0.process(inputs, output)
    }
}

impl<T: Node + Send> Node for BoxedNodeSend<T> {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        self.0.process(inputs, output)
    }
}

impl<T> From<Box<T>> for BoxedNode<T>
where
    T: 'static + Node,
{
    fn from(n: Box<T>) -> Self {
        BoxedNode(n as Box<T>)
    }
}

impl<T> From<Box<T>> for BoxedNodeSend<T>
where
    T: 'static + Node + Send,
{
    fn from(n: Box<T>) -> Self {
        BoxedNodeSend(n as Box<T>)
    }
}

impl<T: Node> fmt::Debug for BoxedNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNode").finish()
    }
}

impl<T: Node + Send> fmt::Debug for BoxedNodeSend<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNodeSend").finish()
    }
}

impl<T: Node> Deref for BoxedNode<T> {
    type Target = Box<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BoxedNodeSend {
    type Target = Box<dyn Node + Send>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<N: Node> DerefMut for BoxedNode<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for BoxedNodeSend {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
