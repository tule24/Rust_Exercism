# Lucian's Luscious Lasagna
---
## Hướng dẫn
---

Trong bài tập này, bạn sẽ viết một chương trình nhằm giúp bạn nấu một món lasagna tuyệt vời từ quyển sách dạy nấu ăn yêu thích của bạn.
Bạn sẽ phải hoàn thành 4 nhiệm vụ, tất cả chúng đều liên quan đến việc tính toán thời gian để nấu món lasagna.

## 1. Define the expected oven time in minutes
---

Viết một hàm `expected_minutes_in_oven` nhằm xác định thời gian cần thiết món lasagna phải ở trong lò để chín. Theo sách dạy nấu ăn, thời gian 
dự kiến là 40 phút
```bash
expected_minutes_in_oven()
// Returns: 40
```

## 2. Calculate the remaining oven time in minutes
---

Viết một hàm `remaining_minutes_in_oven` sẽ nhận đầu vào là thời gian thực tế mà món lasagna đã ở trong lò và đầu ra là số thời gian mà món 
lasagna cần phải tiếp tục ở trong lò cho tới khi chín. 
Ví dụ: 
- Thời gian món ăn cần ở trong lò cho tới khi chín là `40 phút` lấy từ hàm `expected_minutes_in_oven()`
- *Input:* Thời gian thực tế món ăn đã ở trong lò là `30 phút`
- *Output:* Thời gian món ăn phải tiếp tục để ở trong lò cho đến khi chín
```bash
remaining_minutes_in_oven(30)
// Returns: 10
```
## 3. Calculate the preparation time in minutes
---

Viết một hàm `preparation_time_in_minutes` sẽ nhận đầu vào là số nguyên liệu bạn cần cho món lasagna và trả về tổng thời gian bạn dành ra để chuẩn bị nguyên liệu cho món lasagna, giả định mỗi nguyên liệu bạn phải mất 2 phút để chuẩn bị
Ví dụ: 
- ***Input:*** Số nguyên liệu cần chuẩn bị là `2 nguyên liệu`
- ***Output:*** Thời gian chuẩn bị nguyên liệu cho món lasagna
```bash
preparation_time_in_minutes(2)
// Returns: 4
```

## 4. Calculate the elapsed time in minutes
---

Viết một hàm `elapsed_time_in_minutes` sẽ nhận vào hai tham số, tham số thứ nhất là số nguyên liệu của món lasagna, tham số thứ hai là thời gian thực tế món lasagna đã ở trong lò. Hàm này sẽ trả về tổng thời gian bạn **đã bỏ ra** để nấu món lasagna, nó sẽ là tổng thời gian để chuẩn bị các nguyên liệu cộng với thời gian thực tế món lasagna đã ở trong lò.
Ví dụ: 
- ***Input:*** Số nguyên liệu cần chuẩn bị là `3 nguyên liệu`, Thời gian món lasagna đã ở trong lò là `20 phút`
- ***Output:*** Tổng thời gian thực tế bạn đã bỏ ra để nấu món lasagna tính tới hiện tại
```bash
elapsed_time_in_minutes(3, 20)
// Returns: 26
```
## Source: [https://exercism.org/](https://exercism.org/) 