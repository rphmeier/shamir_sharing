Shamir's secret sharing for short secrets (~32 bytes or fewer)

Example usage: divide the secret up into 7 points, any 5 of which can be used to reconstruct the secret. 
```
$ cargo build
$ target/debug/shamir-sharing create -s "6f766e037b9a518c5ba9fa5c44a2291e0c08047c1370c199fe892547dcafc2d" -n 7 -t 5
{"x":"adbc3aaaa2ae49302fbc43729c10b6f5246f25abbb34134dd4055f81d70f7010","y":"94c525dfe703beec21f2f7a8da19292b63b98352cf0a3871f8ee796710214be4"}
{"x":"e2aa6fa7d10f571f63d758df16e7740238d4b62e39c1c27fca7046f54b0c014e","y":"1d5106145ebef19e8b4472c8db3a8aaeee65bd2ac002bfe8eb10e6053f020e6f"}
{"x":"ef54d8b0b6aa452a8941c4f2b9303c259003185100ae66a59365235de26ecb86","y":"bc78ed824e17590ddf6c7946035c5b100a978367618e70925b48b79df25f72f9"}
{"x":"6597d86c2dbcaa87830433966bccad54158db80ce35e55f61694d0097e2ce912","y":"4c4b64604d8170e4657ef7c2cbf5fdff2240b2db115bb5a5e4fad34f5443bcf2"}
{"x":"ab901f21f10eb26790ea4e3ad89a2c7b055f647cfa94a83d1421ffbd729f7f02","y":"66fd1e365176e4c4c395f6dac04decaf67bfcd9e6c8b228db9ab03f56a22cf02"}
{"x":"a39ca28fc39d9a2c292ffaf209440431f641070f2c5b4784a107ae990a1b8b12","y":"227829b268945c12afe840250d81185638f49c8ca398c944fd2de410f36c1a1f"}
{"x":"e430b5015e00cefcf99544576ffb92a15b4a66cce914330c1a891b478852eb10","y":"4b95c5e6f23fa1889de5e175696d1a221cb42fbab068a4d84da52660298e0b65"}
```

Actually reconstructing some of those secrets:

```
$ echo "
{\"x\":\"e2aa6fa7d10f571f63d758df16e7740238d4b62e39c1c27fca7046f54b0c014e\",\"y\":\"1d5106145ebef19e8b4472c8db3a8aaeee65bd2ac002bfe8eb10e6053f020e6f\"}
{\"x\":\"ef54d8b0b6aa452a8941c4f2b9303c259003185100ae66a59365235de26ecb86\",\"y\":\"bc78ed824e17590ddf6c7946035c5b100a978367618e70925b48b79df25f72f9\"}
{\"x\":\"6597d86c2dbcaa87830433966bccad54158db80ce35e55f61694d0097e2ce912\",\"y\":\"4c4b64604d8170e4657ef7c2cbf5fdff2240b2db115bb5a5e4fad34f5443bcf2\"}
{\"x\":\"ab901f21f10eb26790ea4e3ad89a2c7b055f647cfa94a83d1421ffbd729f7f02\",\"y\":\"66fd1e365176e4c4c395f6dac04decaf67bfcd9e6c8b228db9ab03f56a22cf02\"}
{\"x\":\"a39ca28fc39d9a2c292ffaf209440431f641070f2c5b4784a107ae990a1b8b12\",\"y\":\"227829b268945c12afe840250d81185638f49c8ca398c944fd2de410f36c1a1f\"}
" | target/debug/shamir-sharing restore
6f766e037b9a518c5ba9fa5c44a2291e0c08047c1370c199fe892547dcafc2d
```
