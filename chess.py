from gym_chess_pleco import ChessEnv, Board, Piece, PieceType, Square, Player, GenTypes


board = Board()

print(board.fen())
print(board)


def sq_to_coords(sq: str):
    x = ord(sq[0]) - 97
    y = int(sq[1]) - 1
    return x, y


def coords_to_sq(x, y):
    return f"{chr(x+97)}{y+1}"


promotion_map = {
    "Q": 0,
    "B": 1,
    "N": 2,
    "R": 3,
}
promotion_map_reverse = {
    0: "Q",
    1: "B",
    2: "N",
    3: "R",
}


#
# Chess Action space
# ref: https://ai.stackexchange.com/a/6924
def move_to_action(move: str):
    sq_from, sq_to, prom_to = move[:2], move[2:4], move[4:]

    x1, y1 = sq_to_coords(sq_from)
    x2, y2 = sq_to_coords(sq_to)
    dx = x2 - x1
    dy = y2 - y1

    print("dx", dx)
    print("dy", dy)

    origin_sq = x1 * 8 + y1

    # Promotion move
    if prom_to:
        print("it's a promotional move")
        target_piece = promotion_map[prom_to]

        if dx == -1:
            promo_move = 0
        elif dx == 0:
            promo_move = 1
        elif dx == 1:
            promo_move = 2
        else:
            raise Exception("illegal move")

        print("target_piece", target_piece)
        print("promo_move", promo_move)

        promo = target_piece * 4 + promo_move
        print("promo", promo)

        move_type = 8 * 7 + 8 + promo
        print("move_type: ", move_type)
        print("origin_sq: ", origin_sq)
        return move_type + origin_sq * 73

    # Knight move
    if (abs(dx) == 1 and abs(dy) == 2) or (abs(dx) == 2 and abs(dy) == 1):
        print("it's a knight move")
        knight_move_map = {
            (1, 2): 0,
            (2, 1): 1,
            (2, -1): 2,
            (1, -2): 3,
            (-1, -2): 4,
            (-2, -1): 5,
            (-2, 1): 6,
            (-1, 2): 7,
        }
        knight_move = knight_move_map.get((dx, dy))
        print("knight_move", knight_move)
        move_type = 8 * 7 + knight_move
        print("move_type", move_type)
        print("origin_sq: ", origin_sq)
        return move_type + (x1 * 8 + y1) * 73

    print("it's a directional move")
    direction_move_map = {
        (0, 1): 0,
        (1, 1): 1,
        (1, 0): 2,
        (1, -1): 3,
        (0, -1): 4,
        (-1, -1): 5,
        (-1, 0): 6,
        (-1, 1): 7,
    }

    def get_direciton(n: int):
        if n == 0:
            return 0
        elif n < 0:
            return -1
        return 1

    x_dir = get_direciton(dx)
    y_dir = get_direciton(dy)

    print("x_dir", x_dir)
    print("y_dir", y_dir)

    direction = direction_move_map.get((x_dir, y_dir))
    num_steps = max(abs(dx), abs(dy))
    print("direction", direction)
    print("num_steps", num_steps)
    directional_move = direction * 8 + num_steps
    print("directional_move", directional_move)
    print("move_type", directional_move)
    print("origin_sq: ", origin_sq)

    return directional_move + (x1 * 8 + y1) * 73


def action_to_move(action: int):
    print("action:", action)
    origin_sq = action // 73
    move_type = action % 73
    print("origin_sq, move_type ===>", origin_sq, move_type)

    from_x = origin_sq // 8
    from_y = origin_sq % 8

    from_sq = coords_to_sq(from_x, from_y)
    print("from_sq", from_sq)

    # Promotion move
    if move_type >= (73 - 9):
        print("it's a pomo move")
        promo = move_type - (73 - 9)

        promo_target = promo // 4
        promo_move = promo % 4

        target_piece = promotion_map_reverse.get(promo_target)

        to_y = from_y + 1
        dx = 0
        if promo_move == 0:
            dx = 1
        elif promo_move == 1:
            dx = 0
        elif promo_move == 2:
            dx = -1

        to_x = from_x + dx
        to_sq = coords_to_sq(to_x, to_y)

        return from_sq + to_sq + target_piece

    # Knight move
    elif move_type >= (73 - 9 - 8):
        print("it's a knight move")
        knight_move = move_type - (73 - 9 - 8)

        knight_move_map = {
            0: (1, 2),
            1: (2, 1),
            2: (2, -1),
            3: (1, -2),
            4: (-1, -2),
            5: (-2, -1),
            6: (-2, 1),
            7: (-1, 2),
        }
        dx, dy = knight_move_map.get(knight_move)

        to_x = from_x + dx
        to_y = from_y + dy
        to_sq = coords_to_sq(to_x, to_y)
        return from_sq + to_sq

    # Directional move
    print("it's a directional move", move_type)
    direction = move_type // 8
    num_steps = move_type % 8

    print("direction:", direction)
    print("num_steps:", num_steps)

    direction_move_map = {
        0: (0, 1),
        1: (1, 1),
        2: (1, 0),
        3: (1, -1),
        4: (0, -1),
        5: (-1, -1),
        6: (-1, 0),
        7: (-1, 1),
    }
    x_dir, y_dir = direction_move_map.get(direction)

    dx = x_dir * num_steps
    dy = y_dir * num_steps

    to_x = from_x + dx
    to_y = from_y + dy
    to_sq = coords_to_sq(to_x, to_y)
    return from_sq + to_sq


coords = sq_to_coords("a1")
print(coords)
sq = coords_to_sq(5, 4)
print(sq)

x = action_to_move(1)
print(x)

# Promotional
move = "a1a2Q"
action = move_to_action(move)
print("\n>>>", move, "=>", action)

move_2 = action_to_move(action)
print("\n>>>", action, "=>", move_2)

print("\n\n\n\n")

# Knight
move = "b1c3"
action = move_to_action(move)
print("\n>>>", move, "=>", action)

move_2 = action_to_move(action)
print("\n>>>", action, "=>", move_2)

print("\n\n\n\n")

# Directional
move = "d1h1"
action = move_to_action(move)
print("\n>>>", move, "=>", action)

move_2 = action_to_move(action)
print("\n>>>", action, "=>", move_2)


print("\n\n\n\n")

# Directional
move = "a1h8"
action = move_to_action(move)
print("\n>>>", move, "=>", action)

move_2 = action_to_move(action)
print("\n>>>", action, "=>", move_2)
