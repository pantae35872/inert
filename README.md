# This is a software for L.I.S project for thailand innovative innovation 2025 
This use tauri, with svelte frontend and rppal for raspberry pi control, the backend has 2 "backends"
simulate (no raspberry pi just log to the console) or rpi (the real one that uses rppal)

Can be ran with 
```bash 
RPI_RECOGNITION_PATH="$PWD/rpi-recognition" npm run tauri dev -- -- --no-default-features --features sim
```
for simulation
and a normal default features for rpi
```
RPI_RECOGNITION_PATH="$PWD/rpi-recognition" npm run tauri dev
```

# Setting up
```bash
git clone https://github.com/pantae35872/inert.git
cd inert
npm install
cd rpi-recognition
python -m venv venv
./venv/pip -r requirements.txt
cd ../

RPI_RECOGNITION_PATH="$PWD/rpi-recognition" npm run tauri dev # See the first section for more info
```
