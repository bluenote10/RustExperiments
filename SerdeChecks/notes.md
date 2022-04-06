
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