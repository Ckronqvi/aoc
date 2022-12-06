package day3;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.HashSet;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader rd = null;
        try {
            rd = new BufferedReader(new FileReader("src/day3/resources.txt"));
        } catch (FileNotFoundException fe) {
            fe.printStackTrace();
        }
        String line;
        int score = 0;
        HashSet<Character> chars = new HashSet<>();
        // Task 1
        while ((line = rd.readLine()) != null) {
            chars.clear();
            // First half we add chars to hashset.
            for (int i = 0; i < line.length() / 2; i++) {
                chars.add(line.charAt(i));
            }
            for (int i = line.length() / 2; i < line.length(); i++) {
                if (chars.contains(line.charAt(i))) {
                    score += calcScore(line.charAt(i));
                    break;
                }
            }

        }
        rd.close();

        // Task 2
        try {
            rd = new BufferedReader(new FileReader("src/day3/resources.txt"));
        } catch (FileNotFoundException fe) {
            fe.printStackTrace();
        }
        HashSet<Character> chars2 = new HashSet<>();
        HashSet<Character> chars3 = new HashSet<>();
        Character badge = null;
        int score2 = 0;
        // Assumes that the lines count is divisible by 3.
        while ((line = rd.readLine()) != null) {
            chars.clear();
            chars2.clear();
            chars3.clear();

            // First of three
            for (int i = 0; i < line.length(); i++) {
                chars.add(line.charAt(i));
            }
            line = rd.readLine();
            for (int i = 0; i < line.length(); i++) {
                chars2.add(line.charAt(i));
            }

            line = rd.readLine();
            for (int i = 0; i < line.length(); i++) {
                chars3.add(line.charAt(i));
            }

            String str = chars.toString();
            for (int i = 0; i < str.length(); i++) {
                if (chars2.contains(str.charAt(i))) {
                    if (chars3.contains(str.charAt(i))) {
                        badge = str.charAt(i);
                        break;
                    }
                }
            }
            score2 += calcScore((char) badge);
        }

        System.out.format("Task 1 score is: %d%n.", score);
        System.out.format("Task 2 score is: %d.", score2);
        rd.close();
    }

    public static int calcScore(char c) {
        if (Character.isLowerCase(c)) {
            // ASCII value of 'a' is 97, so we subtract 96 to get value of 1.
            return c - 96;
        } else {
            // ASCII value of 'A' is 65, so we subtract 38 to get 27.
            return c - 38;
        }
    }
}