
Initial implementation without any `skip_serializing_if`:

```
orig:                  222920   10.250
json:                  244185   11.227
pretty.json:           553241   25.438
msgpack (compact):      56125    2.581
msgpack (named):       174734    8.034
```

After adding `skip_serializing_if`:

```
orig:                  222920   10.250
json:                  119186    5.480
pretty.json:           295712   13.597
msgpack (compact):      47943    2.204
msgpack (named):        96957    4.458
```

With compression:

```
orig:                    222920   10.250      11642    0.535    2.883
json:                    119186    5.480       9727    0.447    2.409
pretty.json:             295712   13.597      11276    0.518    2.792
msgpack (compact):        47943    2.204       9055    0.416    2.242
msgpack (named):          96957    4.458      10361    0.476    2.566
bincode:                  60440    2.779       9065    0.417    2.245
cbor:                     75809    3.486       9821    0.452    2.432
```

After integer size reduction:

```
orig:                    222920   10.250      11642    0.535    2.883
json:                    119186    5.480       9727    0.447    2.409
pretty.json:             295712   13.597      11276    0.518    2.792
msgpack (compact):        47943    2.204       9055    0.416    2.242
msgpack (named):          96957    4.458      10361    0.476    2.566
bincode:                  41594    1.912       8289    0.381    2.053
cbor:                     75809    3.486       9821    0.452    2.432
bare:                     41289    1.898       8268    0.380    2.048
```

Comparison to custom format:

```
orig:                    222920   10.250      11642    0.535    2.883
json:                    119186    5.480       9727    0.447    2.409
pretty.json:             295712   13.597      11276    0.518    2.792
msgpack (compact):        47943    2.204       9055    0.416    2.242
msgpack (named):          96957    4.458      10361    0.476    2.566
bincode:                  41594    1.912       8289    0.381    2.053
cbor:                     75809    3.486       9821    0.452    2.432
bare:                     41289    1.898       8268    0.380    2.048
custom:                   24494    1.126       7832    0.360    1.940
```
