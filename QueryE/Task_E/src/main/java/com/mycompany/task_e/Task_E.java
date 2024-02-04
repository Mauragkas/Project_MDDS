package com.mycompany.task_e;

import static com.mycompany.task_e.SaveSkylineToJSON.saveSkylinePointsToJsonFile;
import java.util.Arrays;
import java.util.List;
import java.util.Random;
import org.jfree.chart.ChartFactory;
import org.jfree.chart.ChartPanel;
import org.jfree.chart.JFreeChart;
import org.jfree.chart.plot.PlotOrientation;
import org.jfree.data.xy.XYDataset;
import org.jfree.data.xy.XYSeries;
import org.jfree.data.xy.XYSeriesCollection;
import javax.swing.JFrame;

public class Task_E {

    public static void main(String[] args) {
        
        List<Point> points = Arrays.asList
        (
            new Point(4, 76),
            new Point(25, 33),
            new Point(89, 16),
            new Point(95, 24),
            new Point(70, 44),
            new Point(2, 91),
            new Point(86, 18),
            new Point(82, 56),
            new Point(26, 2),
            new Point(10, 74),
            new Point(17, 88),
            new Point(44, 89),
            new Point(96, 61),
            new Point(69, 21),
            new Point(9, 69),
            new Point(5, 68),
            new Point(99, 60),
            new Point(74, 87),
            new Point(66, 90),
            new Point(33, 87),
            new Point(66, 61),
            new Point(75, 37),
            new Point(69, 80),
            new Point(29, 21),
            new Point(29, 29),
            new Point(32, 28),
            new Point(32, 21),
            new Point(88, 20),
            new Point(20, 37),
            new Point(30, 34)
        );
  
        
        List<Point> skylinePoints = Skyline.findSkylinePoints(points);

        System.out.println("Skyline Points: " + skylinePoints);

        JFrame frame = new JFrame("Positive Only Cartesian Level For All Points");
        JFrame frame1 = new JFrame("Positive Only Cartesian Level For Skyline Points");
        

        // Create Dataset
        XYDataset dataset = createDataset(points);
        XYDataset dataset1 = createDataset(skylinePoints);

        // Create Chart
        JFreeChart chart = ChartFactory.createScatterPlot(
            "Cartesian Plot", 
            "X-Axis", "Y-Axis", dataset, PlotOrientation.VERTICAL,
            true, true, false);
        
        JFreeChart chart1 = ChartFactory.createScatterPlot(
            "Cartesian Plot", 
            "X-Axis", "Y-Axis", dataset1, PlotOrientation.VERTICAL,
            true, true, false);

        // Add chart to a panel
        ChartPanel panel = new ChartPanel(chart);
        frame.setContentPane(panel);
        ChartPanel panel1 = new ChartPanel(chart1);
        frame1.setContentPane(panel1);

        // Display the frame
        frame.setSize(600, 400);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setVisible(true);
        
        frame1.setSize(600, 400);
        frame1.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame1.setVisible(true);
        
        String filePath = "skyline_points.json";
        saveSkylinePointsToJsonFile(skylinePoints, filePath);
    }
    
    private static XYDataset createDataset(List<Point> getPoints) {
        XYSeries series = new XYSeries("Data Series");

        for (Point point : getPoints) {
            series.add(point.getX(), point.getY());
        }
        
        
        XYSeriesCollection dataset = new XYSeriesCollection();
        dataset.addSeries(series);

        return dataset;
    }
}
