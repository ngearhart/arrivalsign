FROM python:3.11

RUN mkdir /app
WORKDIR /app

RUN git clone --depth 1 https://github.com/hzeller/rpi-rgb-led-matrix.git
RUN cd rpi-rgb-led-matrix && sed -i '1s/^/HARDWARE_DESC=adafruit-hat-pwm/' lib/Makefile && \
    apt-get update && apt-get install python3-dev python3-pillow -y && \
    make build-python PYTHON=$(command -v python3) && \
    make install-python PYTHON=$(command -v python3)

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .
