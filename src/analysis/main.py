import numpy as np
import matplotlib.pyplot as plt

file_name = input("Type csv file name.csv: ")
chart_title = input("Type chart title: ")


def create_graph_from_csv(csv_file):
    # Initialize empty lists
    x_values = []
    y_values = []
    sort_time = 0

    # Read data from CSV file
    with open(csv_file, "r") as file:
        for line in file:
            parts = line.strip().split(";")
            if parts[0] == "Sort_Time":
                sort_time = float(parts[1])
            else:
                x_values.append(parts[0])
                if parts[0] != "BSC":
                    y_values.append(float(parts[1]) + sort_time)
                else:
                    y_values.append(float(parts[1]))

    # Convert lists to numpy arrays for faster processing
    x_values = np.array(x_values)
    y_values = np.array(y_values)

    plt.annotate(
        "Sort: " + str(sort_time) + "ms",
        xy=(0, sort_time),
        xytext=(0, sort_time),
        ha="center",
        va="bottom",
    )
    plt.bar(x_values, y_values)
    plt.xlabel("Algoritmo")
    plt.ylabel("Tempo em milissegundos")
    plt.title(chart_title)
    plt.grid(True)
    plt.show()


# Example usage
csv_file_path = "../results/"  # Replace with the path to your CSV file
csv_file_path = csv_file_path + file_name
create_graph_from_csv(csv_file_path)
