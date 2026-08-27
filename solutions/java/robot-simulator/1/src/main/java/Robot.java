class Robot {
    private int x;
    private int y;
    private Orientation d;

    Robot(GridPosition initialPosition, Orientation initialOrientation) {
        x = initialPosition.x;
        y = initialPosition.y;
        d = initialOrientation;
    }

    GridPosition getGridPosition() {
        return new GridPosition(x,y);
    }

    Orientation getOrientation() {
        return d;
    }

    void advance() {
        if (d==Orientation.NORTH) {
            y++;
        } else if (d==Orientation.EAST) {
            x++;
        } else if (d==Orientation.SOUTH) {
            y--;
        } else if (d==Orientation.WEST) {
            x--;
        }
    }

    void turnLeft() {
        if (d==Orientation.NORTH) {
            d = Orientation.WEST;
        } else if (d==Orientation.EAST) {
            d = Orientation.NORTH;
        } else if (d==Orientation.SOUTH) {
            d = Orientation.EAST;
        } else if (d==Orientation.WEST) {
            d = Orientation.SOUTH;
        }
    }

    void turnRight() {
        if (d==Orientation.NORTH) {
            d = Orientation.EAST;
        } else if (d==Orientation.EAST) {
            d = Orientation.SOUTH;
        } else if (d==Orientation.SOUTH) {
            d = Orientation.WEST;
        } else if (d==Orientation.WEST) {
            d = Orientation.NORTH;
        }
    }

    void simulate(String instructions) {
        for (char x : instructions.toCharArray()){
            if (x=='R') {
                turnRight();
            } else if (x=='L') {
                turnLeft();
            } else if (x=='A') {
                advance();
            }
        }
    }

}