import pandas as pd  # data manipulation and analysis
import time
import psutil


def load(filepath):
    df = pd.read_csv(filepath)
    return df


def get_data_descriptive_stats(dataframe, column):
    statistics = {
        "Mean": dataframe[column].mean(),
        "Median": dataframe[column].median(),
        "StdDev": dataframe[column].std(),
    }
    return pd.Series(statistics)


if __name__ == "__main__":
    totaltime = 0
    totalmem = 0
    for i in range(1000):
        start = time.time()
        path = "rust_analysis/insurance.csv"
        ins_df = load(path)
        statistics1 = get_data_descriptive_stats(ins_df, "charges")

        end = time.time()
        duration = end - start
        cpu_usage = psutil.cpu_percent()
        mem_usage = psutil.virtual_memory()
        totalmem += mem_usage.percent
        totaltime += duration

    print(statistics1)
    print(f"Elapsed time: {totaltime/1000:.4f} seconds")
    print(f"CPU Usage: {cpu_usage}%")
    print(f"Memory Usage: {round(totalmem/1000,2)}%")
