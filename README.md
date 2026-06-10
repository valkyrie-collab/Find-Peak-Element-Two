```markdown
# 2D Grid Peak Finding Engine in Rust

An optimized, cache-friendly implementation of the **2D Peak Finder** algorithm written in Rust. By conducting a binary search across row slices combined with an exhaustive linear horizontal scan, this engine lowers the operational time complexity from a brute-force $\mathcal{O}(M \times N)$ down to a highly efficient $\mathcal{O}(N \log M)$.

---

## 🏗️ Architectural Strategy & Pipeline

The system uses a row-splitting divide-and-conquer strategy. Instead of jumping arbitrarily through index spaces, it isolates one middle row vector per iteration and utilizes structural pointer lookups to decide exactly which half of the grid to search next.

```text
┌──────────────────────────────────────────────┐
│           Console Matrix Ingestion           │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│            Binary Search Engine              │
├──────────────────────────────────────────────┤
│ - Tracks `left` (Row 0) and `right` bounds   │
│ - Calculates `mid` row address bisector      │
└──────────────────────┬───────────────────────┘
                       │
                       ▼ 
┌──────────────────────────────────────────────┐
│       Linear Peak Isolation (Row Scan)       │
├──────────────────────────────────────────────┤
│ Scans entire `arr[mid]` vector to catch the  │
│ maximum element value and its column (`fxi`) │
└──────────────────────┬───────────────────────┘
                       │
                       ▼ 
┌──────────────────────────────────────────────┐
│       Vertical Neighbor Comparison           │
├──────────────────────────────────────────────┤
│ Looks up `mid - 1` and `mid + 1` elements    │
│ at the targeted column index (`fxi`).        │
└──────────────────────┬───────────────────────┘
                       │
        ┌──────────────┴──────────────┐
        ▼ [Peak Discovered]           ▼ [Slopes Detected]
┌──────────────────────────────┐┌──────────────────────────────┐
│ Return Coordinates [mid, fxi]││ Adjust Row Bounds & Re-cycle │
└──────────────────────────────┘└──────────────────────────────┘

```

---

## 📊 Visualizing the Grid Slope Traversal

To understand how the code moves toward a peak, consider this example grid where a binary search identifies a central row slice.

### The Row-Scanning Peak Invariant:

By scanning an entire row to pick its absolute maximum item, your code establishes a vital property: **The chosen element is already guaranteed to be greater than its immediate left and right neighbors.** This leaves only two vertical paths to verify:

### Grid Neighborhood Assessment Matrix:

Let's look at how the row pointers shift around a sub-array maximum point:

```text
               Col 0      Col 1     Col fxi      Col 4
            ┌──────────┬──────────┬──────────┬──────────┐
 Row mid-1  │    24    │    37    │ [up_val] │     5    │  ◄── If up_val > max: right = mid - 1
            ├──────────┼──────────┼──────────┼──────────┤
 Row mid    │     3    │    43    │  [max]   │    19    │  ◄── max is the absolute row peak
            ├──────────┼──────────┼──────────┼──────────┤
 Row mid+1  │     6    │    15    │[down_val]│    18    │  ◄── If down_val > max: left = mid + 1
            └──────────┴──────────┴──────────┴──────────┘

```

* **The Gradient Principle:** If `up_val` is greater than `max`, a rising slope travels upward. Because `up_val` is strictly greater than `max`, and `max` is greater than any item in its own row, a valid peak is mathematically guaranteed to exist somewhere in the top half of the grid.

---

## 🔬 In-Depth Code Walkthrough

### 1. The Row Bisector Split

```rust
mid = left + (right - left) / 2;
let sub_arr: &Vec<i32> = &arr[mid as usize];

```

* **Overflow Protection:** Using `left + (right - left) / 2` calculates the mid-point without risks of arithmetic integer overflow.
* **Slice Borrowing:** Instead of deep-copying vector contents, `&arr[mid as usize]` registers a fast, lightweight reference borrow to let the system read the data without heap reallocation penalty.

### 2. Micro-Scan Vector Maximum Evaluation

```rust
let mut fxi: i32 = -1;
let mut max: i32 = i32::MIN;

for (xi, &xe) in sub_arr.iter().enumerate() {
    max = if xe > max { fxi = xi as i32; xe } else { max };
}

```

* **`enumerate()` Tracking:** The index loop maps out column position addresses (`xi`) alongside array values (`xe`).
* **Condition Assignment:** The inline ternary evaluation continually rewires the target register state, shifting the final column address into `fxi`.

### 3. Perimeter Boundary Handling

```rust
if mid - 1 == -1 {
    up_val = -1;
} else {
    up_val = arr[mid as usize - 1][fxi as usize];
}

```

* **Virtual Guard Rails:** To keep runtime loops inside strict vector safety allocations, your logic manually detects out-of-bounds attempts (`mid - 1 == -1`). If a boundary is touched, it automatically provisions a default value of `-1` to represent the outer perimeter environment described by the problem statement.

### 4. Convergence Routing

```rust
if max > up_val && max > down_val {
    return [mid, fxi];
} else if max < up_val && max > down_val {
    right = mid - 1;
} else {
    left = mid + 1;
}

```

* **Terminal Node Found:** When `max` outperforms both its top and bottom counterparts, a peak is logged, and execution loops break instantly.
* **Binary Reduction:** If it fails, the search window boundaries are moved inward past `mid`, discarding exactly half of the remaining rows from any subsequent evaluations.

---

## ⏱️ Complexity Profile

### Time Complexity: $\mathcal{O}(N \log M)$

* **Calculation:** Let $M$ represent rows and $N$ represent columns. The binary search operates on the row bounds, running a maximum of $\log M$ times. In every iteration, the system loops over the full length of a row, taking $\mathcal{O}(N)$ steps. This yields an optimal, lightning-fast total runtime profile.

### Space Complexity: $\mathcal{O}(1)$

* **Storage Footprint:** Pointer variables (`mid`, `fxi`, `max`) are strictly placed on the local thread stack. No supplementary mirror matrices or storage vectors are generated, keeping auxiliary runtime footprint constant.

```

```
