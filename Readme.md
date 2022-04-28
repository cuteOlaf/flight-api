# Flight Path Tracking API

## Goal

Create a microservice API that can help understand and track how a particular person's flight path may be queried.


## Required Structure
* **[['SFO', 'EWR']]** => **['SFO', 'EWR']**
* **[['ATL', 'EWR'], ['SFO', 'ATL']]** => **['SFO', 'EWR']**
* **[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']]** => **['SFO', 'EWR']**

## API endpoint
With [Postman](https://www.getpostman.com/)

- `/index`
  - method: `GET`
  - url: `https://127.0.0.1:8080/` (or `https://localhost:8080`)
- `/flight`
  - method: `POST`
  - url: `https://127.0.0.1:8080/flight` (or `https://localhost:8080/flight`)
  - header : `Content-Type` = `application/json`
  - body(raw): `[["SFO", "EWR"]]`
## Framework
[Actix](https://actix.rs/) - Web framework for Rust
## Input Validation Check
- Validate `JSON data`
    ```json
    // should fail
    [[["from", "to"]]]
    [["A", "B"], ["C", "D"], {"key": "value"}]
    ```
- Only **one** in, **one** out for every airport
    ```rust
    if flight_from.contains_key(destination) || flight_to.contains_key(source) {
        return Err(error::ErrorBadRequest("Invalid input"));
    }
    ```

- Only one start, one end for the overall flight path
    ```rust
    let mut current = flight_start.to_string();
    let mut count = 1;
    loop {
        if current.eq(flight_end) {
            break;
        }
        match flight_to.get(&current) {
            None => {
                return Err(error::ErrorBadRequest("Invalid input"));
            }
            Some(&airport) => {
                current = airport.into();
            }
        }
        count = count + 1;
    }
    if count != airports.len() {
        return Err(error::ErrorBadRequest("Invalid input"));
    }
    ```