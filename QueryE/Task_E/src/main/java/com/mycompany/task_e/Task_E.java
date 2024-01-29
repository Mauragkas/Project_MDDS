package com.mycompany.task_e;

import java.util.Arrays;
import java.util.List;
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
        List<Point> points = Arrays.asList(
            new Point(1, 2),
            new Point(3, 4),
            new Point(5, 3),
            new Point(2, 6),
            new Point(4, 1)
        );

        List<Point> skylinePoints = Skyline.findSkylinePoints(points);

        System.out.println("Skyline Points: " + skylinePoints);

        JFrame frame = new JFrame("Positive Only Cartesian Level");

        // Create Dataset
        XYDataset dataset = createDataset(skylinePoints);

        // Create Chart
        JFreeChart chart = ChartFactory.createScatterPlot(
            "Cartesian Plot", 
            "X-Axis", "Y-Axis", dataset, PlotOrientation.VERTICAL,
            true, true, false);

        // Add chart to a panel
        ChartPanel panel = new ChartPanel(chart);
        frame.setContentPane(panel);

        // Display the frame
        frame.setSize(600, 400);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setVisible(true);
    }
    
    private static XYDataset createDataset(List<Point> skylinePoints) {
        XYSeries series = new XYSeries("Data Series");

        for (Point point : skylinePoints) {
            series.add(point.getX(), point.getY());
        }

        XYSeriesCollection dataset = new XYSeriesCollection();
        dataset.addSeries(series);

        return dataset;
    }
}
