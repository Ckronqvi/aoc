package day2;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader rd = null;
        int score = 0;
        int score2 = 0;
        String line;
        try {
            rd = new BufferedReader(new FileReader("src/day2/resource.txt"));
        } catch (FileNotFoundException fe) {
        }

        while ((line = rd.readLine()) != null) {
            line = line.replaceAll("\s+", "");
            score += getPoints(line.charAt(1));
            score += getOutcome(line.charAt(0), line.charAt(1));
            score2 += getNeededChoiceAndPoints(line.charAt(0), line.charAt(1));
        }
        System.out.println("Task 1 score: " + score);
        System.out.println("Task 2 score: " + score2);
        rd.close();
    }

    // For task 2.
    public static int getNeededChoiceAndPoints(char opp, char outcome) {
        switch (opp) {
            case 'A':// Rock
                if (outcome == 'X') { // Lose
                    return 3 + 0; // Scissors + losing points
                }
                if (outcome == 'Y') { // Draw
                    return 1 + 3; // Rock + Draw points
                }
                if (outcome == 'Z') { // Win
                    return 2 + 6; // Paper + Win points
                }
                break;

            case 'B':// Paper
                if (outcome == 'X') { // Lose
                    return 1 + 0; // Rock + losing points
                }
                if (outcome == 'Y') { // Draw
                    return 2 + 3; // Paper + Draw points
                }
                if (outcome == 'Z') { // Win
                    return 3 + 6; // Scissors + Win points
                }
                break;

            case 'C':// Scissors
                if (outcome == 'X') { // Lose
                    return 2 + 0; // Paper + losing points
                }
                if (outcome == 'Y') { // Draw
                    return 3 + 3; // Scissors + Draw points
                }
                if (outcome == 'Z') { // Win
                    return 1 + 6; // Rock + Win points
                }
                break;
        }
        return 0;
    }

    public static int getPoints(char c) {
        switch (c) {
            case 'X': // Rock
                return 1;
            case 'Y': // Paper
                return 2;
            case 'Z': // Sciccors
                return 3;
        }
        return 0;
    }

    // For task 1
    public static int getOutcome(char opp, char self) {
        switch (opp) {
            case 'A': // Rock
                if (self == 'X') {
                    return 3; // Draw
                }
                if (self == 'Y') {
                    return 6; // Won
                }
                break;
            case 'B': // Paper
                if (self == 'Y') {
                    return 3; // Draw
                }
                if (self == 'Z') {
                    return 6; // Won
                }
                break;
            case 'C': // Scissors
                if (self == 'Z') {
                    return 3; // Draw
                }
                if (self == 'X') {
                    return 6; // Won
                }
                break;
        }
        return 0; // Lost
    }
}
