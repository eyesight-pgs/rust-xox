
fn print_board(board: [char; 10]) {
    println!("");
    println!(" {} | {} | {} ", board[1], board[2], board[3]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[4], board[5], board[6]);
    println!("---|---|---");
    println!(" {} | {} | {} ", board[7], board[8], board[9]);
    println!("");
}
fn req_ip(player_id: char) {
    println!("Player {}'s turn: ", player_id);
}
fn toggle_player(current_player: char) -> char {
    match current_player {
        'x' => return 'o',
        'o' => return 'x',
        'X' => return 'O',
        'O' => return 'X',
        _ => todo!(),
    }
}
fn is_winner(player: char, board: [char; 10]) -> (bool, [i32; 3]) {
    let win_sets: [[i32; 3]; 8] = [
        [1,2,3],
        [4,5,6],
        [7,8,9],
        [1,4,7],
        [2,5,8],
        [3,6,9],
        [1,5,9],
        [3,5,7],
    ];
    for set in win_sets {
        let mut count: i32 = 0;
        for i in set {
            // println!("testing for loop: {:?}", set);
            if board[i as usize] != player {
                break;
            }
            count += 1;
        }
        if count == 3 {
            return (true, set);
        }
    }
    return (false, [-1, -1, -1]);
}
fn display_winner(set: [i32; 3], player: char, board: &mut [char; 10]) {
    println!("");
    println!("=============================");
    println!("PLAYER {} IS WINNER!", player);
    for i in set {
        board[i as usize] = board[i as usize].to_uppercase().collect::<Vec<_>>()[0];
    }
    print_board(*board);
    println!("=============================");
}

fn main() {
    let mut board: [char; 10] = [
        ' ', // ignore 0th index
        ' ', ' ', ' ',
        ' ', ' ', ' ',
        ' ', ' ', ' ',
    ];
    let mut player: char = 'x';
    print_board(board);
    loop {
        req_ip(player);
        let mut cell: String = String::new();
        std::io::stdin().read_line(&mut cell);
        cell = cell.trim().to_string();
        let i: usize = cell.parse().unwrap();
        board[i] = player;
        print_board(board);
        let (flag, set) = is_winner(player, board);
        // println!("is {} winner? {}", player, flag);
        if flag {
            display_winner(set, player, &mut board);
            break;
        }
        player = toggle_player(player);
    }
    
}



