package day1;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Collections;

public class Main {
    public static void main(String[] args) {
        BufferedReader rd = null;
        try {
            rd = new BufferedReader(new FileReader("src/day1/calories.txt"));
        } catch (FileNotFoundException fe) {
            System.out.println("File not found. Too bad...");
        }
        try {
            List<Integer> list = new ArrayList<>();
            String line;
            int calSum = 0;
            while ((line = rd.readLine()) != null) {
                if (!line.matches(".*\\d+.*")) {
                    list.add(calSum);
                    calSum = 0;
                } else {
                    calSum += Integer.parseInt(line);
                }
            }
            list.add(calSum);
            list.sort(Collections.reverseOrder());
            int sum = list.get(0) + list.get(1) + list.get(2);
            System.out.format("Maximum calories: %d%n", list.get(0));
            System.out.format("Maximum calories of 3 elves: %d", sum);
        } catch (IOException ioe) {
        }
    }
}