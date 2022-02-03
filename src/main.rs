use rand::Rng;
use std::io;
fn main() {
    //setup maze vector
    let mut size = String::new();
    print!("Enter size of maze: ");
    io::stdin().read_line(&mut size).expect("Failed to read line");
    let size:usize = size.trim().parse().expect("USE A NUMBER!");

    let mut maze: Vec<Vec<char>> = vec![vec!['#';size]; size];
    
    
    
    gen_path(&mut maze, size);

    
}
fn reset_maze(maze:&mut Vec<Vec<char>>){
    for vec in maze{
        for cha in vec{
            *cha = '#';
        }
    }
}
fn print_maze(maze: & Vec<Vec<char>>){
    println!();
    for vec in maze{
        for cha in vec{
            print!("{} ", cha);
        }
        println!("");
    }
}
fn maze_to_hashtag(maze: &mut  Vec <Vec<char>>){
    for vec in maze{
        for cha in vec{
            if *cha == '#'{
                *cha = 'M';
            }
        }
    }
}
fn m_to_n(maze: &mut Vec<Vec<char>>) {
    for vec in maze{
        for cha in vec{
            if *cha == 'M'{
                *cha = 'N';
            }
        }
    }
}
fn gen_path(maze: &mut Vec<Vec<char>>, size:usize) {
//keep running till we have a possble path
    let mut row:usize;
    let mut col:usize = size / 2;
    let end_row:usize = 0;
    let end_col:usize = col;
    let mut error:bool = true;

    while error{
        reset_maze(maze);
        row = size - 1;
        col = size / 2;
        maze[row][col] = '0';
        maze[end_row][col] = 'E';
        error = false;
        while !error && !wincheck(&mut row, &mut col, maze){
            //scan maze
            scan(maze, end_row, end_col);
            //check possble movements
            check_possible(&mut error, &mut row, &mut col, maze);
            //move

            //wincheck
        }
        
        if  wincheck(&mut row, &mut col, maze){
            println!("WIN!");
            print_maze(maze);
        }
        
    }
    


}
fn move_it(f_row:usize, f_col:usize, maze:&mut Vec<Vec<char>>, row:&mut usize, col:&mut usize){
    maze[f_row][f_col] = ' ';
    *row = f_row;
    *col = f_col;
}
fn check_neighbours(row:usize, col:usize, maze: &mut Vec<Vec<char>>) -> bool{
    let mut count:usize = 0;
    for i in 0..3{
        for j in 0..3{
            if row >= 1 && col >= 1{
                if row - 1 + i >= maze.len()|| col - 1 + j >= maze.len(){break;}

                if maze[row - 1 + i][col -1 + j] == ' ' && !(i == 1 && j == 1){
                    count = count + 1;
                }
            } 
            
        }
    }
    

    return if count <= 2{true}else{false};
}

fn check_possible(error:&mut bool, row:&mut usize, col:&mut usize, maze: &mut Vec<Vec<char>>){

    //vars
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    
    //check possble movements
        //up
        if *row >= 1{
            if maze[*row - 1][*col] == '#' && check_neighbours(*row, *col, maze){
                up = true;
            }
        }
        
        if *row + 1 < maze.len(){
            if maze[*row + 1][*col] == '#' && check_neighbours(*row, *col, maze){
                down = true;
            }
        }
        if *col + 1 < maze.len(){
            if maze[*row][*col + 1] == '#' && check_neighbours(*row, *col, maze){
                right = true;
            }
        }
        if *col >= 1{
            if maze[*row][*col - 1] == '#' && check_neighbours(*row, *col, maze){
                left = true;
            }
        }
        
        //println!("{}, {}, {}, {}", up,down,left,right);
        if (!up) && (!down) && (!left) && (!right) {*error = true; return}
        let mut random_int:usize;
        loop{
            random_int = rand::thread_rng().gen_range(0..4);
            //println!("{}o", random_int);
            match random_int{

                0 => if up {move_it(*row - 1, *col, maze, row, col); return},
                1 => if down {move_it(*row + 1, *col, maze, row, col); return},
                2 => if left {move_it(*row, *col - 1, maze, row, col); return},
                3 => if right {move_it(*row, *col + 1, maze, row, col); return}
                _ => ()
            }
        }

    
}


fn wincheck(row:&mut usize, col:&mut usize, maze:&mut Vec<Vec<char>>) -> bool{
    for i in 0..3{
        for j in 0..3{
            if *row >= 1 && *col >= 1{
                if *row - 1 + i >= maze.len()|| *col - 1 + j >= maze.len(){break;}
                if maze[*row - 1 + i][*col -1 + j] == 'E'{
                    return true;
                }
            }
            
        }
    }
    return false;
}

fn scan(maze:&mut Vec<Vec<char>>, end_row:usize, end_col:usize){
    maze_to_hashtag(maze);
    m_eater(maze, end_row + 1, end_col);
    m_eater(maze, end_row, end_col - 1);
    m_eater(maze, end_row, end_col + 1);
    m_to_n(maze);
}

fn m_eater(maze:&mut Vec<Vec<char>>, row:usize, col:usize){
    if row >= 1{
        if maze[row - 1][col] == 'M' && check_neighbours(row - 1, col, maze){
            maze[row - 1][col] = '#';
            m_eater(maze, row - 1, col);
        }
    }
    
    if row + 1 < maze.len(){
        if maze[row + 1][col] == 'M' && check_neighbours(row + 1, col, maze){
            maze[row + 1][col] = '#';
            m_eater(maze, row + 1, col);
        }
    }
    if col >= 1{
        if maze[row][col - 1] == 'M' && check_neighbours(row, col - 1, maze){
            maze[row][col - 1] = '#';
            m_eater(maze, row, col - 1);
        }
    }
   
    if col + 1 < maze.len(){
        if maze[row][col + 1] == 'M' && check_neighbours(row, col + 1, maze){
            maze[row][col + 1] = '#';
            m_eater(maze, row, col + 1);
        }
    }
    
}