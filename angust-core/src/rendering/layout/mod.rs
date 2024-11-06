/*
 * Layout system, consisting of two stages:
 *   a. A leaves -> root traversal of the DOM, estimating sizes of the elements
 *   b. A root -> leaves pass that recursively allocates positions and sizes,
 * based on the estimated children sizes and parent constraints.
 */

pub mod space_allocation_system;
pub mod size_estimation_system;