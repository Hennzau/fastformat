import matplotlib.pyplot as plt

# Données
sizes = [0xfd200, 0x2a3000, 0x5eec00, 0x17bb000]

# Latence en microsecondes (µs)
latency_fastformat = [433.785, 388.052, 261.482, 221.552]
latency_raw_arrow = [337.045, 305.905, 278.438, 187.905]

# Débit (messages par seconde)
throughput_fastformat = [5318, 2318, 987, 225]
throughput_raw_arrow = [5823, 2360, 969, 125]

# Convertir les tailles en hexadécimal pour les labels
size_labels = [hex(s) for s in sizes]

# Tracé des graphes
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

# Graphique de latence
ax1.plot(size_labels, latency_fastformat, marker='o', color='blue', label='fastformat arrow')
ax1.plot(size_labels, latency_raw_arrow, marker='o', color='red', label='raw arrow')
ax1.set_title("Latency Comparison")
ax1.set_xlabel("Size")
ax1.set_ylabel("Latency (µs)")
ax1.legend()
ax1.grid(True)

# Graphique de débit
ax2.plot(size_labels, throughput_fastformat, marker='o', color='blue', label='fastformat arrow')
ax2.plot(size_labels, throughput_raw_arrow, marker='o', color='red', label='raw arrow')
ax2.set_title("Throughput Comparison")
ax2.set_xlabel("Size")
ax2.set_ylabel("Throughput (messages per second)")
ax2.legend()
ax2.grid(True)

plt.tight_layout()

plt.savefig("latency_throughput_comparison.png", format='png')
plt.show()
