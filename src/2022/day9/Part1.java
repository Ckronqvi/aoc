package day9;

import java.io.BufferedReader;
import java.io.FileReader;

public class Part1 {
    static int headX = 0, headY = 0;
    static int tailX = 0, tailY = 0;

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
        headX = headY = startPos;
        tailX = tailY = startPos;
        grid[tailY][tailX] = new GridPosition();
        tailVisitedCounter++;

        for (int i = 0; i < movement.length; i++) {
            for (int j = 0; j < Integer.parseInt(movement[i][1]); j++) {
                moveHead(movement[i][0].charAt(0));
                moveTail();
                if (grid[tailY][tailX] == null) {
                    grid[tailY][tailX] = new GridPosition();
                    tailVisitedCounter++;
                }
            }
        }
        System.out.format("Visited count: %d", tailVisitedCounter);

    }

    private static void moveHead(char dir) {
        switch (dir) {
            case 'U':
                headY--;
                break;
            case 'D':
                headY++;
                break;
            case 'L':
                headX--;
                break;
            case 'R':
                headX++;
                break;
        }
    }

    private static void moveTail() {

        if (Math.abs(headX - tailX) > 1) {
            if (headX > tailX) {
                tailX++;
            } else {
                tailX--;
            }
            if (headY != tailY) {
                tailY = headY;
            }
        }

        if (Math.abs(headY - tailY) > 1) {
            if (headY > tailY) {
                tailY++;
            } else {
                tailY--;
            }
            if (headX != tailX) {
                tailX = headX;
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