"""A scalable chess board with mouse, drag and keyboard input."""
import chess
import chess.svg
from PySide6.QtCore import QByteArray, QPointF, QRectF, Qt, Signal
from PySide6.QtGui import QColor, QFont, QPainter, QPen
from PySide6.QtSvg import QSvgRenderer
from PySide6.QtWidgets import QSizePolicy, QWidget


class BoardWidget(QWidget):
    move_requested = Signal(int, int)
    square_focused = Signal(str)

    def __init__(self):
        super().__init__()
        self.board = chess.Board()
        self.flipped = False
        self.guides = True
        self.interactive = True
        self.selected = None
        self.cursor = chess.E2
        self.hint = None
        self.press_square = None
        self.press_pos = None
        self.drag_pos = None
        self.accent = QColor("#b3cb92")
        self.dark = QColor("#697d68")
        self.light = QColor("#dce1cf")
        self.ink = QColor("#e4e8df")
        self.pieces = {piece.symbol(): QSvgRenderer(QByteArray(chess.svg.piece(piece).encode()))
                       for color in chess.COLORS for kind in chess.PIECE_TYPES
                       for piece in [chess.Piece(kind, color)]}
        self.setFocusPolicy(Qt.FocusPolicy.StrongFocus)
        self.setMinimumSize(320, 320)
        self.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
        self.setAccessibleName("Chess board")
        self.setAccessibleDescription("Arrow keys move between squares. Enter selects a piece or destination. Escape clears selection. You can also enter moves in the move field.")

    def geometry_for_board(self):
        side = max(1, min(self.width(), self.height()) - 44)
        return (self.width() - side) / 2, (self.height() - side) / 2, side / 8

    def cell(self, square):
        x, y, size = self.geometry_for_board()
        col, row = chess.square_file(square), 7 - chess.square_rank(square)
        if self.flipped:
            col, row = 7 - col, 7 - row
        return QRectF(x + col * size, y + row * size, size, size)

    def square_at(self, point):
        x, y, size = self.geometry_for_board()
        col, row = int((point.x() - x) // size), int((point.y() - y) // size)
        if not (0 <= col < 8 and 0 <= row < 8):
            return None
        if self.flipped:
            col, row = 7 - col, 7 - row
        return chess.square(col, 7 - row)

    def set_position(self, board, interactive):
        self.board, self.interactive = board.copy(), interactive
        self.selected = self.hint = self.drag_pos = None
        self.update()

    def focus_square(self, square):
        self.cursor = square
        piece = self.board.piece_at(square)
        description = chess.square_name(square) + (f", {'white' if piece.color else 'black'} {chess.piece_name(piece.piece_type)}" if piece else ", empty")
        self.square_focused.emit(description)
        self.update()

    def activate(self, square):
        self.focus_square(square)
        if not self.interactive:
            return
        piece = self.board.piece_at(square)
        if piece and piece.color == self.board.turn:
            self.selected = None if self.selected == square else square
        elif self.selected is not None:
            source = self.selected
            self.selected = None
            self.move_requested.emit(source, square)
        self.update()

    def mousePressEvent(self, event):
        if event.button() != Qt.MouseButton.LeftButton:
            return
        self.setFocus()
        self.press_square = self.square_at(event.position())
        self.press_pos = event.position()

    def mouseMoveEvent(self, event):
        if self.press_square is None or not self.interactive:
            return
        piece = self.board.piece_at(self.press_square)
        if piece and piece.color == self.board.turn and (event.position() - self.press_pos).manhattanLength() > 8:
            self.selected = self.press_square
            self.drag_pos = event.position()
            self.update()

    def mouseReleaseEvent(self, event):
        if event.button() != Qt.MouseButton.LeftButton:
            return
        target = self.square_at(event.position())
        dragged = self.drag_pos is not None
        source = self.press_square
        self.drag_pos = self.press_square = None
        if target is not None:
            if dragged and target != source:
                self.selected = None
                self.move_requested.emit(source, target)
            elif not dragged:
                self.activate(target)
        self.update()

    def keyPressEvent(self, event):
        key = event.key()
        moves = {Qt.Key.Key_Left: (-1, 0), Qt.Key.Key_Right: (1, 0),
                 Qt.Key.Key_Up: (0, 1), Qt.Key.Key_Down: (0, -1)}
        if key in moves:
            dx, dy = moves[key]
            if self.flipped:
                dx, dy = -dx, -dy
            file = max(0, min(7, chess.square_file(self.cursor) + dx))
            rank = max(0, min(7, chess.square_rank(self.cursor) + dy))
            self.focus_square(chess.square(file, rank))
        elif key in (Qt.Key.Key_Return, Qt.Key.Key_Enter, Qt.Key.Key_Space):
            self.activate(self.cursor)
        elif key == Qt.Key.Key_Escape:
            self.selected = None
            self.update()
        else:
            super().keyPressEvent(event)

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)
        last = self.board.peek() if self.board.move_stack else None
        targets = {m.to_square for m in self.board.legal_moves if m.from_square == self.selected} if self.guides else set()
        for square in chess.SQUARES:
            rect = self.cell(square)
            painter.fillRect(rect, self.dark if (chess.square_file(square) + chess.square_rank(square)) % 2 == 0 else self.light)
            if last and square in (last.from_square, last.to_square):
                painter.fillRect(rect, QColor(220, 211, 106, 100))
            if self.board.is_check() and square == self.board.king(self.board.turn):
                painter.fillRect(rect, QColor(220, 72, 60, 150))
            if square == self.selected:
                painter.fillRect(rect, QColor(240, 220, 105, 145))
            piece = self.board.piece_at(square)
            if piece and not (self.drag_pos is not None and square == self.selected):
                pad = rect.width() * 0.1
                self.pieces[piece.symbol()].render(painter, rect.adjusted(pad, pad, -pad, -pad))
            if square in targets:
                painter.setPen(Qt.PenStyle.NoPen)
                painter.setBrush(QColor(20, 35, 25, 100))
                radius = rect.width() * (0.12 if not piece else 0.43)
                if piece:
                    painter.setBrush(Qt.BrushStyle.NoBrush)
                    painter.setPen(QPen(QColor(20, 35, 25, 120), 4))
                painter.drawEllipse(rect.center(), radius, radius)
            if square == self.cursor and self.hasFocus():
                painter.setPen(QPen(QColor("#15221a"), 2, Qt.PenStyle.DashLine))
                painter.setBrush(Qt.BrushStyle.NoBrush)
                painter.drawRect(rect.adjusted(4, 4, -4, -4))
        if self.hint:
            painter.setPen(QPen(self.accent, 5))
            painter.drawLine(self.cell(self.hint.from_square).center(), self.cell(self.hint.to_square).center())
            painter.setBrush(self.accent)
            painter.drawEllipse(self.cell(self.hint.to_square).center(), 7, 7)
        if self.drag_pos is not None and self.selected is not None:
            piece = self.board.piece_at(self.selected)
            size = self.cell(self.selected).width()
            if piece:
                self.pieces[piece.symbol()].render(painter, QRectF(self.drag_pos.x()-size/2, self.drag_pos.y()-size/2, size, size))
        painter.setPen(self.ink)
        painter.setFont(QFont("sans-serif", 10))
        x, y, size = self.geometry_for_board()
        for index in range(8):
            file = 7-index if self.flipped else index
            rank = index+1 if self.flipped else 8-index
            painter.drawText(QRectF(x+index*size, y+size*8+3, size, 18), Qt.AlignmentFlag.AlignCenter, chess.FILE_NAMES[file])
            painter.drawText(QRectF(x-21, y+index*size, 17, size), Qt.AlignmentFlag.AlignCenter, str(rank))
