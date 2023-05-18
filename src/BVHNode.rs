use crate::Shape::{Shape, Hittable};
use crate::BoundingBox;
pub struct BVHNode
{
    leftChild: Vec<Shape>,
    rightChild: Vec<Shape>,
    bounding_box: BoundingBox
}

impl BVHNode
{
    pub fn new(Shapes: Vec<Shape>, axis: u8) -> BVHNode
    {


        if Shapes.len() == 1
        {
            let r: Vec<Shape> = Vec::new();
            BVHNode{leftChild: Shapes,rightChild: r, bounding_box: Shapes[0].getBoundingBox()}
        }
        else if Shapes.len() == 2
        {
            let mut l: Vec<Shape> = Vec::new();
            l.push(Shapes[0]);

            let mut r: Vec<Shape> = Vec::new();
            r.push(Shapes[1]);

            let bbox = Shapes[0].getBoundingBox().union(Shapes[1].getBoundingBox());
            BVHNode{leftChild: l,rightChild: r, bounding_box: bbox}
        }
        else 
        {
            let tempBox = Shapes[0].getBoundingBox();
            for shape in Shapes.iter()
            {
                tempBox = tempBox.union(shape.getBoundingBox());
            }
            let bbox = tempBox;

            //split based on axis

            let split = (bbox.minPt + bbox.maxPt) / 2.0;

            let leftShapes: Vec<Shape> = Vec::new();
            let rightShapes: Vec<Shape> = Vec::new();

            for shape in Shapes
            {
                if shape.getCentroid()[axis as usize] > split[axis as usize]
                {
                    rightShapes.push(shape);
                }
                else 
                {
                    leftShapes.push(shape);
                }
            }

            BVHNode{leftChild: leftShapes,rightChild: rightShapes, bounding_box: bbox}
            //BVHNode::new(rightShapes, (axis + 1) % 3)





        }

        
    }
}