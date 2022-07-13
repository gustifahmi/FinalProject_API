# FinalProject_API

Nama: Gusti Fahmi Fadhila  
Kode Peserta: KSAT006ONL012

## API Test
API yang ditest adalah sebagai berikut:  

### 1. Auth
   1) https://restful-booker.herokuapp.com/auth POST (Create Token)  

### 2. Booking
   1) https://restful-booker.herokuapp.com/booking POST (Create Booking)  

   2) https://restful-booker.herokuapp.com/booking/${bookingid} DELETE (Delete Booking)  
      Test ini hanya berhasil HANYA JIKA dijalankan di Test Suite, karena menggunakan bookingid yang diset di Global Variable saat Create Booking. Namun, jika dijalankan di Test Suite atau Object Repository akan gagal karena menggunakan default Global Variable bookingid yang sudah terhapus. Alternatif lainnya adalah mengecek GetBookingIds dan menggunakan salah satu id yang ada di response tersebut.

   3) https://restful-booker.herokuapp.com/booking/137 GET (Get Booking)  
      Jika akun sudah terhapus, bisa menggunakan id lain yang ada di Get Booking Ids.

   4) https://restful-booker.herokuapp.com/booking GET (Get Booking Ids)  

   5) https://restful-booker.herokuapp.com/booking/95 PATCH (Partial Update Booking)  
      Jika akun sudah terhapus, bisa menggunakan id lain yang ada di Get Booking Ids.

   6) https://restful-booker.herokuapp.com/booking/95 PUT (Update Booking)  
      Jika akun sudah terhapus, bisa menggunakan id lain yang ada di Get Booking Ids.

### 3. Ping
   1) https://restful-booker.herokuapp.com/ping GET (Health Check)