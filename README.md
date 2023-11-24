## Build

```
cargo build --release
```

## Usage

### 1. Hex to Limb Conversion

#### 256 Bits

- #### 32-bit word

```
./target/release/u256 32 <HEX>
```

- #### 64-bit word

```
./target/release/u256 64 <HEX>
```

#### 384 Bits

- #### 32-bit word

```
./target/release/u381 32 <HEX>
```

- #### 64-bit word

```
./target/release/u381 64 <HEX>
```

### 2. 64 bits to 32 bits limb conversion

```
./target/release/limb_convert <l_0 l_1 l_2 .. l_n>
```