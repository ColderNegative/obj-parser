use std::{fs, vec};

#[derive(Debug)]
struct Object3d {
    path: String,
    faces: Vec<Face>
}

struct RenderedTriangle {
    vectors: Vec<Point2d>,
    normal: f32
}

#[derive(Debug)]
struct Face {
    vectors: Vec<Point3d>,
    normal: Point3d
}

#[derive(Clone, Debug)]
struct Point3d {
    x: f32,
    y: f32,
    z: f32
}

struct Point2d {
    x: f32,
    y: f32
}

impl Object3d {
    fn new(path: String, faces: Vec<Face>) -> Self {
        Self {
            path,
            faces
        }
    }

    // TODO: implement method
    fn rotate(&mut self, rotation_x: f32, rotation_y: f32, rotation_z: f32) {
        let matrix_x: Vec<Vec<f32>> = vec![
            vec![1.0, 0.0, 0.0], 
            vec![0.0, rotation_x.cos(), -rotation_x.sin()],
            vec![0.0, rotation_x.sin(), rotation_x.cos()] 
        ];

        let matrix_y: Vec<Vec<f32>> = vec![
            vec![rotation_y.cos(), 0.0, rotation_y.sin()], 
            vec![0.0, 1.0, 0.0],
            vec![-rotation_y.sin(), 0.0, rotation_y.cos()] 
        ];

        let matrix_z: Vec<Vec<f32>> = vec![
            vec![rotation_z.cos(), -rotation_z.sin(), 0.0], 
            vec![rotation_z.sin(), rotation_z.cos(), 0.0],
            vec![0.0, 0.0, 1.0] 
        ];


    }

    // TODO: implement method
    fn translate(&mut self, move_x: f32, move_y: f32, move_z: f32) {
        for face in &mut self.faces {
            for point in &mut face.vectors {
                (*point).x += move_x;
                (*point).y += move_y;
                (*point).z += move_z;
            }
        }
    }
}

impl RenderedTriangle {
    fn new(vectors: Vec<Point2d>, normal: f32) -> Self {
        Self {
            vectors, 
            normal
        }
    }
}

impl Face {
    fn new(vectors: Vec<Point3d>, normal: Point3d) -> Self {
        Self {
            vectors,
            normal
        }
    }
}

impl Point3d {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl Point2d {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}

// TODO: implement function 
fn render(obj: Object3d) -> Vec<RenderedTriangle> {
    let mut rendered_triangles: Vec<RenderedTriangle> = Vec::new();
    rendered_triangles
}


fn parse_data(path: String) -> Object3d {
    fn extract_nums(line: &str) -> Point3d{
        let mut nums = line[2..].split_whitespace();
        let num_x = nums.next().unwrap().parse::<f32>().unwrap();
        let num_y = nums.next().unwrap().parse::<f32>().unwrap();
        let num_z = nums.next().unwrap().parse::<f32>().unwrap();
        let point = Point3d::new(num_x, num_y, num_z);
        point
    } 

    fn create_face(line: &str, vectors: &Vec<Point3d>, normals: &Vec<Point3d>) -> Face {
        let mut faces = line[2..].split_whitespace();
        let normal_index = line.chars().last().unwrap().to_digit(10).unwrap() as usize -1;
        let mut face_vectors: Vec<Point3d> = Vec::new();
        for vec in faces {
            let vec_num = vec.chars().next().unwrap().to_digit(10).unwrap() as usize -1;
            face_vectors.push(vectors[vec_num].clone());
        }  
        let face = Face::new(face_vectors, normals[normal_index].clone());
        face
    }

    let mut vectors: Vec<Point3d> = Vec::new(); 
    let mut normals: Vec<Point3d> = Vec::new(); 
    let mut faces = Vec::new(); 
    let contents = fs::read_to_string(&path)
        .expect("Failed to read file or missing file");
    for line in contents.lines() {
        if line.len() < 2 {
            continue;
        }
        let first_chars = &line[0..2];
        match first_chars {
            "v " => vectors.push(extract_nums(&line)),
            "vn" => normals.push(extract_nums(&line)),
            "f " => faces.push(create_face(&line, &vectors, &normals)),
            _ => (),
        }
    }
    let obj = Object3d::new(path, faces);
    obj
}

fn multiply_matrix(array1: &Vec<Vec<f32>>, array2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = vec![vec![0.0f32; array1[0].len()]; array2.len()]; 
    for i in 0..result[0].len() {
        for j in 0..result.len() {
            let mut sum: f32 = 0.0;
            for k in 0..array1.len() {
                sum += array1[i][k] * array2[k][j];
            }
            result[i][j] = sum;
        }
    }
    result
}

fn main() {
    let mut cube = parse_data("src/cube.obj".to_string());
    println!("{:?}", cube);
    let rendered_cube = render(cube);
}


