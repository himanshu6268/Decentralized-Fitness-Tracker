
# Decentralized Fitness Tracker

## 📌 Project Title
**Decentralized Fitness Tracker**

## 📝 Project Description
A blockchain-based fitness logging system that empowers users to record, own, and manage their workout data on-chain. This project ensures transparency, privacy, and verifiable health records while enabling fitness rewards systems in the future.

## 🌍 Project Vision
To revolutionize health and wellness data tracking by allowing users full control over their activity records using decentralized technology.

## ✨ Key Features
- Users can securely log fitness activities like running, cycling, and workouts.
- Fitness data is stored on-chain for verifiable and tamper-proof records.
- Easy retrieval of workout history for progress tracking or health integration.

## 🔍 Contract Details

### Contract Address: CDZFCO4ASIJ2ONSHQ4VL4TUE66IJMJH7SIBC5W3ISSOMOK7YAXOIJGVC

### `log_activity`
- Records a fitness activity with duration, calories, and timestamp.
- Parameters:
  - `user: Address`
  - `activity: String` (e.g., "Running")
  - `duration_minutes: u32`
  - `calories_burned: u32`

### `get_activities`
- Retrieves all logged activities for a given user.
- Parameter:
  - `user: Address`
- Returns:
  - `Vec<ActivityEntry>`

---

*Built using [Soroban SDK](https://soroban.stellar.org/docs)* 🚀
