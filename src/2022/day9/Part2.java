package day9;

import java.io.BufferedReader;
import java.io.FileReader;

public class Part2 {

    public static void main(String[] args) throws Exception {
        GridPosition[][] grid;
        String[][] movement = new String[2000][2];
        int tailVisitedCounter = 0;

        BufferedReader rd = new BufferedReader(new FileReader("src/day9/resource.txt"));
        String line;
        int index = 0;
        while (null != (line = rd.readLine())) {
            movement[index++] = line.split(" ");
        }
        rd.close();

        int size = getGridSize(movement);
        grid = new GridPosition[size][size]; // maybe a bit overkill:
        int startPos = size / 2;
        Node head = new Node(startPos, startPos);
        Node tail = null;
        // Initialize the linked list.
        for (int i = 0; i < 9; i++) {
            Node n = head;
            while (n.next != null) {
                n = n.next;
            }
            n.next = new Node(startPos, startPos);
        }
        Node n = head;
        while (n.getNext() != null) {
            n = n.getNext();
        }
        tail = n;

        grid[startPos][startPos] = new GridPosition();
        tailVisitedCounter++;

        for (int i = 0; i < movement.length; i++) {
            for (int j = 0; j < Integer.parseInt(movement[i][1]); j++) {
                moveHead(movement[i][0].charAt(0), head);
                n = head;
                while (n.getNext() != null) {
                    react(n);
                    n = n.getNext();
                }
                if (grid[tail.getY()][tail.getX()] == null) {
                    grid[tail.getY()][tail.getX()] = new GridPosition();
                    tailVisitedCounter++;
                }
            }
        }
        System.out.format("Visited count: %d", tailVisitedCounter);

    }

    private static void moveHead(char dir, Node head) {
        switch (dir) {
            case 'U':
                head.y--;
                break;
            case 'D':
                head.y++;
                break;
            case 'L':
                head.x--;
                break;
            case 'R':
                head.x++;
                break;
        }
    }

    private static void react(Node node) {
        Node next = node.getNext();
        if (Math.abs(node.getX() - next.getX()) > 1) {
            if (node.x > next.x) {
                next.x++;
            } else {
                next.x--;
            }
            if (Math.abs(node.y - next.y) > 0) {
                if (Math.abs(node.y - next.y) > 1) {
                    if (node.y > next.y) {
                        next.y++;
                    } else {
                        next.y--;
                    }
                } else {
                    next.y = node.y;
                }
            }
        }

        if (Math.abs(node.y - next.y) > 1) {
            if (node.y > next.y) {
                next.y++;
            } else {
                next.y--;
            }
            if (Math.abs(node.x - next.x) > 0) {
                if (Math.abs(node.x - next.x) > 1) {
                    if (node.x > next.x) {
                        next.x++;
                    } else {
                        next.x--;
                    }
                } else {
                    next.x = node.x;
                }
            }
        }
    }

    public static int getGridSize(String[][] movement) {
        int sum = 0;
        for (int i = 0; i < movement.length; i++) {
            sum += Integer.parseInt(movement[i][1]);
        }
        return sum;
    }

}

class GridPosition {

    public GridPosition() {
    }

}

class Node {
    int x, y;
    Node next;

    Node(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public void setX(int x) {
        this.x = x;
    }

    public void setY(int y) {
        this.y = y;
    }

    public void setNext(Node next) {
        this.next = next;
    }

    public Node getNext() {
        return next;
    }
}