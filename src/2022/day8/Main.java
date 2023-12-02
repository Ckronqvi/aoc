package day8;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    final static String fileName = "src/day8/resource.txt";
    final static int length = countLineLengh(fileName);
    final static int height = countLines(fileName);

    public static void main(String[] args) throws IOException {

        BufferedReader rd = null;
        int[][] trees = new int[height][length];
        try {
            rd = new BufferedReader(new FileReader(fileName));
        } catch (FileNotFoundException fe) {
            fe.printStackTrace();
        }
        // Fill the array.
        int i = 0;
        int j = 0;
        String line;
        while (rd.ready()) {
            line = rd.readLine();
            for (int k = 0; k < line.length(); k++) {
                trees[j][i++] = line.charAt(k);
                if (i >= length) {
                    i = 0;
                    j++;
                }
            }
        }

        // check each tree.
        int treeCount = 0;
        boolean foundBigger = false;
        for (int k = 0; k < height; k++) {
            for (int l = 0; l < length; l++) {
                if (!isOnTheEdge(k, l)) {
                    foundBigger = false;
                    // First lets check all the trees to the left
                    for (int ll = l - 1; ll >= 0 && !foundBigger; ll--) {

                        if (trees[k][ll] >= trees[k][l]) {
                            foundBigger = true;
                        }
                    }

                    if (foundBigger) {
                        foundBigger = false;
                        for (int up = k - 1; up >= 0 && !foundBigger; up--) {
                            if (trees[up][l] >= trees[k][l]) {
                                foundBigger = true;
                            }
                        }
                    }

                    if (foundBigger) {
                        foundBigger = false;
                        for (int rr = l + 1; rr < length && !foundBigger; rr++) {
                            if (trees[k][rr] >= trees[k][l]) {
                                foundBigger = true;
                            }
                        }
                    }

                    if (foundBigger) {
                        foundBigger = false;
                        for (int dd = k + 1; dd < height && !foundBigger; dd++) {
                            if (trees[dd][l] >= trees[k][l]) {
                                foundBigger = true;
                            }
                        }
                    }
                    // After all the checks.
                    if (foundBigger) {
                        treeCount++;
                    }
                }
            }
        }
        System.out.format("Visible trees: %d%n", 99 * 99 - treeCount); // All trees - hidden ones.
        System.out.format("Highest score: %d", calcVisibility(trees));
    }

    public static boolean isOnTheEdge(int k, int l) {
        return (k == 0 || k == height - 1) || (l == 0 || l == length - 1);
    }

    // Counts the lines.
    public static int countLines(String fileName) {

        int lines = 0;
        try (BufferedReader reader = new BufferedReader(new FileReader(fileName))) {
            while (reader.readLine() != null) {
                lines++;
            }
            reader.close();
        } catch (IOException e) {
        }
        return lines;
    }

    // Counts line lenght.
    public static int countLineLengh(String fileName) {
        String line;

        try (BufferedReader reader = new BufferedReader(new FileReader(fileName))) {
            line = reader.readLine();
            reader.close();
            return line.length();
        } catch (IOException e) {
        }
        return 100;
    }

    public static int calcVisibility(int[][] trees) {
        int highest = 0;
        int temp = 0;
        int temp2 = 0;
        for (int k = 0; k < height; k++) {
            for (int l = 0; l < length; l++) {
                boolean foundBigger = false;
                temp = 1;
                temp2 = 0;
                for (int ll = l - 1; ll >= 0 && !foundBigger; ll--) {
                    temp2++;
                    if (trees[k][ll] >= trees[k][l]) {
                        foundBigger = true;
                    }
                }

                if (temp2 != 0) {
                    temp *= temp2;
                    temp2 = 0;
                }

                foundBigger = false;
                for (int up = k - 1; up >= 0 && !foundBigger; up--) {
                    temp2++;
                    if (trees[up][l] >= trees[k][l]) {
                        foundBigger = true;
                    }
                }

                if (temp2 != 0) {
                    temp *= temp2;
                    temp2 = 0;
                }

                foundBigger = false;
                for (int rr = l + 1; rr < length && !foundBigger; rr++) {
                    temp2++;
                    if (trees[k][rr] >= trees[k][l]) {
                        foundBigger = true;
                    }
                }

                if (temp2 != 0) {
                    temp *= temp2;
                    temp2 = 0;
                }
                foundBigger = false;
                for (int dd = k + 1; dd < height && !foundBigger; dd++) {
                    temp2++;
                    if (trees[dd][l] >= trees[k][l]) {
                        foundBigger = true;
                    }
                }

                if (temp2 != 0) {
                    temp *= temp2;
                    temp2 = 0;
                }

                if (temp > highest) {
                    highest = temp;
                }

            }
        }
        return highest;
    }

}
