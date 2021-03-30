import math, random, copy, time, numpy as np

EXPLORATION_PARAMETER = math.sqrt(2)

class Board:
    def __init__(self, current_player=1, state=None, prev_move=None):
        self.current_player = current_player
        if state is None:
            self.state = np.zeros((6, 7), dtype='uint8')
        else:
            self.state = state

        self.prev_move = prev_move

        self.winner = self.check_for_winner()
        if self.winner != 0:
            self.is_terminal = True
        else:
            self.is_terminal = False

    def into_move(self, mv):
        new_state = self.state.copy()
        idx = (self.state[:, mv] == 0).argmax()
        new_state[idx, mv] = self.current_player
        return Board(3 - self.current_player, new_state, mv)

    def get_legal_moves(self):
        return list(filter(lambda mv: np.sum(np.where(self.state[:, mv] > 0)) < 6, range(7)))
        
    def set_winner(self, player):
        self.winner = player
        self.is_terminal = True
        return player

    # very brute-force way of determining winners
    def check_for_winner(self):
        is_empty = False
        for i in range(len(self.state)):
            for j in range(len(self.state[i])):
                player = self.state[i][j]
                if player == 0:
                    is_empty = True
                    continue
                try:
                    # horizontally connected
                    if self.state[i+1][j] == player and self.state[i+2][j] == player and self.state[i+3][j] == player:
                        return self.set_winner(player)
                    # vertically connected
                    if self.state[i][j+1] == player and self.state[i][j+2] == player and self.state[i][j+3] == player:
                        return self.set_winner(player)
                    # diagonally north-east connected
                    if self.state[i+1][j+1] == player and self.state[i+2][j+2] == player and self.state[i+3][j+3] == player:
                        return self.set_winner(player)
                    # diagonally north-west connected
                    if self.state[i-1][j+1] == player and self.state[i-2][j+2] == player and self.state[i-3][j+3] == player:
                        return self.set_winner(player)
                except IndexError:
                    pass
        if is_empty:
            return 0
        # draw conditon
        self.winner = -1
        self.is_terminal = True
        return -1

    def __str__(self):
        D = { 0: '.', 1: 'P', 2: 'O' }
        line = ''
        for row in range(len(self.state)-1, -1, -1):
            for _, val in enumerate(self.state[row]):
                line += '%s ' % D.get(val)
            line += '\n'
        return line

class Node:
    def __init__(self, board, parent=None):
        self.board = board
        self.parent = parent
        self.children = list()
        self.win_score = 0
        self.count = 0
        self.is_expanded = False

    def calc_ucb(self):
        if self.count == 0: return float('inf')
        return self.win_score / self.count + EXPLORATION_PARAMETER * math.sqrt(math.log(self.parent.count) / self.count)

class MCTS:
    def __init__(self, root):
        self.root = root

    def search(self, iterations):
        utility_table = { 1: 0, 2: 1, -1: 0.5 }
        for i in range(iterations):
            node = self.root

            # selection
            while node.is_expanded:
                node = self.select(node)

            # expansion
            self.expand(node)
            node = random.choice(node.children)

            # rollout
            winner = self.rollout(node)
            score = utility_table.get(winner, 0)

            # backpropagation
            self.backpropagate(node, score)

        best_move = max(self.root.children, key=lambda n: n.win_score / n.count)
        return best_move.board.prev_move

    def select(self, node):
        if len(node.children) == 0: return None

        return sorted(node.children, key=lambda n: n.calc_ucb())[-1]

    def expand(self, node):
        if not node.is_expanded:
            for move in node.board.get_legal_moves():
                node.children.append(Node(node.board.into_move(move), node))
                node.is_expanded = True

    def rollout(self, node):
        temp_board = copy.deepcopy(node.board)
        while not temp_board.is_terminal:
            move = random.choice(temp_board.get_legal_moves())
            temp_board = temp_board.into_move(move)
            temp_board.current_player = 3 - temp_board.current_player
            temp_board.check_for_winner()
        return temp_board.winner

    def backpropagate(self, node, score):
        while node is not None:
            node.win_score += score
            node.count += 1
            node = node.parent

n_its = 1000
board = Board(current_player=1)

while not board.is_terminal:
    print(board)
    if board.current_player == 1:
        col = int(input("Enter move (1-7): ")) - 1
        board = board.into_move(col)
    else:
        start_time = time.time()
        move = MCTS(Node(board)).search(n_its)
        print("%d iterations took %.3f" % (n_its, 1000*(time.time() - start_time)))
        print("AI chooses column {}".format(move))
        board = board.into_move(move)
print(board)
print("Player {} wins!".format(board.winner))