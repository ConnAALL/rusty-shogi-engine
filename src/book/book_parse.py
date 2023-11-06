def square_to_index(square):
    # Mapping from squares to their index values
    square_to_index_map = {
        '1a': 1, '2a': 10, '3a': 19, '4a': 28, '5a': 37, '6a': 46, '7a': 55, '8a': 64, '9a': 73,
        '1b': 2, '2b': 11, '3b': 20, '4b': 29, '5b': 38, '6b': 47, '7b': 56, '8b': 65, '9b': 74,
        '1c': 3, '2c': 12, '3c': 21, '4c': 30, '5c': 39, '6c': 48, '7c': 57, '8c': 66, '9c': 75,
        '1d': 4, '2d': 13, '3d': 22, '4d': 31, '5d': 40, '6d': 49, '7d': 58, '8d': 67, '9d': 76,
        '1e': 5, '2e': 14, '3e': 23, '4e': 32, '5e': 41, '6e': 50, '7e': 59, '8e': 68, '9e': 77,
        '1f': 6, '2f': 15, '3f': 24, '4f': 33, '5f': 42, '6f': 51, '7f': 60, '8f': 69, '9f': 78,
        '1g': 7, '2g': 16, '3g': 25, '4g': 34, '5g': 43, '6g': 52, '7g': 61, '8g': 70, '9g': 79,
        '1h': 8, '2h': 17, '3h': 26, '4h': 35, '5h': 44, '6h': 53, '7h': 62, '8h': 71, '9h': 80,
        '1i': 9, '2i': 18, '3i': 27, '4i': 36, '5i': 45, '6i': 54, '7i': 63, '8i': 72, '9i': 81
    }
    # Convert the square notation to the index
    index = square_to_index_map.get(square, -1)  # Return -1 if the square is not found
    return index

def parse_move(move, turn):
    # This function parses the move into the proper format
    promote = 'false'
    drop = 'false'
    
    if move[1] == '*':  # Drop move, e.g., 'P*8e'
        piece = move[0]
        to_square = square_to_index(move[2:])
        drop = 'true'
        move_string = f'{turn},{piece},{to_square},{promote},{drop}'
    else:  # Regular move, e.g., '7g7f'
        from_square = square_to_index(move[:2])
        to_square = square_to_index(move[2:4])
        if len(move) == 5:  # Promotion is indicated by a '+' sign
            promote = 'true'
        move_string = f'{turn},{from_square},{to_square},{promote},{drop}'
    
    return move_string

def parse_openings(file_name):
    with open(file_name, 'r') as file:
        lines = file.readlines()

    # Initialize a list to hold formatted moves
    formatted_openings = []

    # Parse each opening line
    for line in lines:
        moves = line.strip().split()
        formatted_line = []
        turn = 'W'  # Start with White

        for move in moves:
            formatted_move = parse_move(move, turn)
            formatted_line.append(formatted_move)
            # Switch turns
            turn = 'B' if turn == 'W' else 'W'

        # Append the formatted line to the list of formatted openings
        formatted_openings.append(' '.join(formatted_line))

    return formatted_openings

def write_formatted_openings(formatted_openings, output_file_name):
    with open(output_file_name, 'w') as file:
        for opening in formatted_openings:
            file.write(opening + '\n')

# Read and parse the openings from the file
formatted_openings = parse_openings("/Users/russell/research/rusty-shogi-engine/src/book/shortopenings.txt")

# Write the formatted openings to a new file
write_formatted_openings(formatted_openings, "/Users/russell/research/rusty-shogi-engine/src/book/formatted_openings.txt")
