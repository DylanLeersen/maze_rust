use rand::Rng;
#[derive(PartialEq, Eq)]
enum direction{
    up,down,left,right
}
fn check_but(dir:direction, row:usize, col:usize, maze:&mut Vec<Vec<char>>) -> bool{
    for i in 0..3{
        for j in 0..3{
            let no:direction;
            if i == 1{
                if j == 0{
                    no = direction::left;
                }else{
                    no = direction::right;
                }
            }else{
                if i == 0{
                    no = direction::up;
                }else{
                    no = direction::down;
                }

            }
            
            if no == dir{break;}
            if i == 0 && j == 0 || i == 0 && j == 3 || i == 3 && j == 0 || i == 3 && j == 3{break;}
            if row >= 1 && col >= 1{
                if row - 1 + i >= maze.len()|| col - 1 + j >= maze.len(){break;}

                if maze[row - 1 + i][col -1 + j] == ' '{
                    return false;
                }
            } 
            
        }
    }
    true
}
fn check_options(mut row:usize, mut col:usize, maze: &mut Vec<Vec<char>>, mut max: usize)-> usize{
    let mut up:bool;
    let mut down:bool;
    let mut left:bool;
    let mut right:bool;
    let mut loop_count = 0;

    loop{
        up = false;
        down = false;
        left = false;
        right = false;
        
        

        if row + 1 < maze.len(){
            if maze[row + 1][col] == '#' && check_neighbours(row, col + 1, maze, max) && if loop_count == 2 {check_but(direction::up, row, col, maze)}else{true}{
                down = true;
            }
        }
        if row >= 1{
            if maze[row - 1][col] == '#' && check_neighbours(row, col + 1, maze, max) && if loop_count == 2{check_but(direction::down, row, col, maze)}else{true}{
                up = true;
            }
        }
        if col >= 1{
            if maze[row][col - 1] == '#' && check_neighbours(row, col + 1, maze, max) && if loop_count == 2{check_but(direction::right, row, col, maze)}else{true}{
                left = true;
            }
        }
        if col + 1 < maze.len(){
            if maze[row][col + 1] == '#'  && check_neighbours(row, col + 1, maze, max) && if loop_count == 2{check_but(direction::left, row, col, maze)}else{true}{
                right = true;
            }
        }

        let mut random_int:usize;
        let num_row: usize;
        let num_col: usize;
        if !(up || down || left || right){
            return loop_count;
        }
        
        loop{
            random_int = rand::thread_rng().gen_range(0..4);
            
            match random_int{
                0 => if up {num_row = row - 1;  num_col = col; move_to(&mut row, &mut col, num_row, num_col, maze); if loop_count == 0 {branch_protection(direction::up, num_row, num_col, maze);}break},
                1 => if down {num_row = row + 1; num_col = col; move_to(&mut row, &mut col, num_row, num_col, maze); if loop_count == 0 {branch_protection(direction::down, num_row, num_col, maze);} break},
                2 => if left {num_row = row; num_col = col - 1; move_to(&mut row, &mut col, num_row, num_col, maze); if loop_count == 0 {branch_protection(direction::left, num_row, num_col, maze);} break},
                3 => if right {num_row = row; num_col = col + 1; move_to(&mut row, &mut col, num_row, num_col, maze); if loop_count == 0 {branch_protection(direction::right, num_row, num_col, maze);} break},
                _ => ()
            }
        }
        if max == 3 {max = 1};
        loop_count += 1;
    }
    
}

fn move_to(row:&mut usize, col:&mut usize, to_row: usize, to_col:usize, maze: &mut Vec<Vec<char>>){
    maze[to_row][to_col] = ' ';
    *row = to_row; 
    *col = to_col; 
    
}
fn branch_protection(dir:direction, row:usize, col:usize, maze: &mut Vec<Vec<char>>){
match dir{
    direction::up => {if col >= 1{if maze[row][col - 1] == '#'{maze[row][col - 1] = 'N'}}; if col + 1 < maze.len(){if maze[row][col + 1] == '#'{maze[row][col + 1] = 'N'}}},
    direction::down => {if col >= 1{if maze[row][col - 1] == '#'{maze[row][col - 1] = 'N'}}; if col + 1 < maze.len(){if maze[row][col + 1] == '#'{maze[row][col + 1] = 'N'}}},
    direction::left => {if row >= 1{if maze[row - 1][col] == '#'{maze[row - 1][col] = 'N'}}; if row + 1 < maze.len(){if maze[row + 1][col] == '#'{maze[row + 1][col] = 'N'}}},
    direction::right => {if row >= 1{if maze[row - 1][col] == '#'{maze[row - 1][col] = 'N'}}; if row + 1 < maze.len(){if maze[row + 1][col] == '#'{maze[row + 1][col] = 'N'}}},
} 
}


pub fn fill(maze:&mut Vec<Vec<char>>, start_row:usize, start_col:usize){
    while follow_lines(start_row, start_col, maze){
        
    }
    
}

fn follow_lines(start_row:usize, start_col:usize, maze: &mut Vec<Vec<char>>) -> bool{
    let mut row:usize = 0;
    let mut col:usize = 0;
    let mut total_loop_count = 0;
    for i in 0..maze.len(){
        for j in 0..maze.len(){
            if maze[i][j] == ' '{
                total_loop_count += check_options(row, col, maze, 1);
                
                
            }
            col += 1;
        }
        row += 1;
        col = 0;
}

return if total_loop_count < 1{false}else{true}
}

fn check_neighbours(row:usize, col:usize, maze: &mut Vec<Vec<char>>, max:usize) -> bool{
    let mut count:usize = 0;
    if row >= 1 && col < maze.len(){
        //up
        if maze[row - 1][col] == ' '{
            count += 1;
        }
    }
    if row + 1 < maze.len() && col < maze.len(){
        //down
        if maze[row + 1][col] == ' '{
            count += 1;
        }
    }
    if col >= 1{
        //left
        if maze[row][col - 1] == ' '{
            count += 1;
        }
    }
    if col + 1< maze.len(){
        //right
        if maze[row][col + 1] == ' '{
            count += 1;
        }
    }
    

    return if count <= max{true}else{false};
}


