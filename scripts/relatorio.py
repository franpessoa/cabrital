import subprocess
import os
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import datetime

fname = f"./sim{datetime.datetime.now()}.csv".replace(" ", "")
output = subprocess.run(["./build/src/cabrito", "-o", fname, "-f", "./config.toml"]).check_returncode()

df = pd.read_csv(fname)
dfm = df.melt('Meses', var_name='Coluna', value_name='Valor')

#sns.relplot(x="Meses", y="Valor", hue="Coluna", kind="line", data = dfm.query('Coluna != "Saída"'))
sns.relplot(x="Meses", y="Valor", hue="Coluna", kind="line", data = dfm)
plt.savefig(fname.replace("csv", "png"))
