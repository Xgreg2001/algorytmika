import matplotlib.pyplot as plt
import pandas as pd

data = pd.read_csv('results.csv')

data.plot(x='n', kind='line')

plt.title('Approximation error vs sample size')

plt.savefig('plot.pdf')
