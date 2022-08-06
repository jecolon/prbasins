# prbasins
Displays Puerto Rico water basin levels and status.

Rust version of Edwood Ocasio's https://github.com/eocasio/embalses

Data provided by the CienciaDatosPR group of the University of Puerto Rico
Humacao Campus Math Department. Data is subject to revision by the USGS.

## Usage
Just a simple Rust project:

```
$ git clone https://github.com/jecolon/prbasins
$ cd prbasins
$ cargo run
```

Sample output:

```
---------------------------------------------------------------------------------------------------------------------
|Basin       | Level  | Change |       Date       |  Status  | Overflow| Secure |Observe | Adjust |Control |Capacity|
---------------------------------------------------------------------------------------------------------------------
|Carraizo    |   40.75|   -0.01| 2022-08-06 10:45 |  SECURE  |    41.14|   39.50|   38.50|   36.50|   30.00|   12000|
|Patillas    |   64.61|   -0.01| 2022-08-06 10:45 | OBSERVE  |    67.07|   66.16|   64.33|   60.52|   59.45|    9890|
|Fajardo     |   52.51|   -0.01| 2022-08-06 10:30 | OVERFLOW |    52.50|   48.30|   43.40|   37.50|   26.00|    4430|
|Caonillas   |  248.64|   -0.01| 2022-08-06 10:45 |  SECURE  |   252.00|  248.00|  244.00|  242.00|  235.00|   31730|
|Carite      |  542.61|   -0.01| 2022-08-06 10:40 |  SECURE  |   544.00|  542.00|  539.00|  537.00|  536.00|    8320|
|Guajataca   |  191.18|   -0.02| 2022-08-06 10:45 | OBSERVE  |   196.00|  194.00|  190.00|  186.00|  184.00|   33340|
|Cerrillos   |  155.17|   -0.03| 2022-08-06 10:45 |  ADJUST  |   173.40|  160.00|  155.50|  149.40|  137.20|   42600|
|Cidra       |  401.33|   -0.00| 2022-08-06 10:45 |  SECURE  |   403.05|  401.05|  400.05|  399.05|  398.05|    4480|
|Rio Blanco  |   28.76|   -0.00| 2022-08-06 10:30 | OVERFLOW |    28.75|   26.50|   24.25|   22.50|   18.00|    3795|
|Toa Vaca    |  139.99|   -0.02| 2022-08-06 10:15 |  ADJUST  |   161.00|  152.00|  145.00|  139.00|  133.00|   50650|
|La Plata    |   44.47|   -0.05| 2022-08-06 10:45 |  SECURE  |    51.00|   43.00|   39.00|   38.00|   31.00|   26516|
---------------------------------------------------------------------------------------------------------------------
```

