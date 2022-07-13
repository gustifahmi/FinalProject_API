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
   3) https://restful-booker.herokuapp.com/booking/137 GET (Get Booking)
   4) https://restful-booker.herokuapp.com/booking GET (Get Booking Ids)
   5) https://restful-booker.herokuapp.com/booking/95 PATCH (Partial Update Booking)
   6) https://restful-booker.herokuapp.com/booking/95 PUT (Update Booking)  

### 3. Ping
   1) https://restful-booker.herokuapp.com/ping GET (Health Check)