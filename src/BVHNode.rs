
use std::sync::{Arc};

use crate::Shape::{Shape, Hittable};
use crate::{BoundingBox, HStruct, Vec3};

#[derive(Clone)]
pub struct BVHNode
{
    leftChild: Option<Arc<Shape>>,
    rightChild: Option<Arc<Shape>>,
    bounding_box: BoundingBox
}

impl BVHNode
{
    pub fn create_empty() -> BVHNode
    {
        BVHNode { leftChild: None, rightChild: None, bounding_box: BoundingBox::new(Vec3::newEmpty(), Vec3::newEmpty()) }
    }

    pub fn new(Shapes: &[Arc<Shape>], axis: i32) -> BVHNode
    {
        Self::build_bvh_node(Shapes.to_vec(),axis)
    }

    fn build_bvh_node(Shapes: Vec<Arc<Shape>>, axis: i32) ->BVHNode
    {
        if Shapes.is_empty()
        {
            BVHNode::create_empty()
        }
        else if Shapes.len() == 1
        {
            let shape =Arc::clone(&Shapes[0]);
            let shape_ref: &Shape = Arc::as_ref(&shape);
            BVHNode{leftChild: Some(Arc::clone(&shape)), rightChild: None, bounding_box: shape_ref.getBoundingBox()}
        }
        else if Shapes.len() == 2
        {
            let l = Arc::clone(&Shapes[0]);
            let r = Arc::clone(&Shapes[1]);
            let bbox = Shapes[0].getBoundingBox().union(Shapes[1].getBoundingBox());
            BVHNode{leftChild: Some(l),rightChild: Some(r), bounding_box: bbox}
        }
        else 
        {
            
            let mut bbox = Shapes[0].getBoundingBox();
            for shape in Shapes.to_vec()
            {
                bbox = bbox.union(shape.getBoundingBox());
                
            }
            //split based on axis
            let split = (bbox.minPt + bbox.maxPt) / 2.0;
            let mut leftShapes: Vec<Arc<Shape>> = Vec::new();
            let mut rightShapes: Vec<Arc<Shape>> = Vec::new();

            for shape in Shapes
            { 
                if shape.getCentroid()[axis as usize] > split[axis as usize]
                {
                    rightShapes.push(Arc::clone(&shape));
                }
                else 
                {
                    leftShapes.push(Arc::clone(&shape));
                }
            }
            
            
            let LB = Self::build_bvh_node(rightShapes, (axis+1) % 3);
            let RB = Self::build_bvh_node(leftShapes, (axis+1) % 3);
            let Ls = Shape::BVHNode(LB);
            let Rs = Shape::BVHNode(RB);
            let L = Some(Arc::new(Ls));
            let R = Some(Arc::new(Rs));
            BVHNode{leftChild: L, rightChild: R, bounding_box: bbox}
        }

    }
}

impl Hittable for BVHNode
{
    fn closestHit(&mut self ,r: &crate::Ray::Ray , tMin: f32 , tMax: f32,h: &mut HStruct ) -> bool 
    {
        let mut leftHit: bool = false;
        let mut rightHit: bool = false;
        let mut left_t: f32 = tMax;
        let mut right_t: f32 = tMax;
        let mut left_normal = Vec3::newEmpty();
        let mut left_s = "".to_string();
        let mut left_i = Vec3::newEmpty();

        if self.bounding_box.intersect(r)
        {
            if self.leftChild.is_some()
            {
                let mut shape = self.leftChild.clone().unwrap().as_ref().clone();
                leftHit = shape.closestHit(r, tMin, tMax, h);
                if h.getT() < tMax
                {
                    left_t = h.getT();
                    left_normal = h.getNormal();
                    left_s = h.getShaderName();
                    left_i = h.getIntersect();
                } 
            }
            if self.rightChild.is_some()
            {
                let mut shape = self.rightChild.clone().unwrap().as_ref().clone();
                rightHit = shape.closestHit(r, tMin, tMax, h); 
                if h.getT() < tMax
                {
                    right_t = h.getT();
                }
            }

            if leftHit && rightHit
            {
               
                if right_t < left_t
                {
                    return true;
                }
                else {
                    h.setT(left_t);
                    h.setNormal(left_normal);
                    h.setShaderName(left_s);
                    h.setIntersect(left_i);
                    return true;
                }
                
            }
            else if leftHit
            {
                return true;
            }
            else if rightHit
            {

                return true;
            }
            else {
                return false;
            }
        }          
        return false;
    }

    fn getBoundingBox(&self) -> BoundingBox {
        self.bounding_box
    }
    fn getCentroid(&self) -> crate::Vec3 {
        Vec3::newEmpty()
    }
    fn getShaderName(&self) -> String {
        "".to_string()
    }
}