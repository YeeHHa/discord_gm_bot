FROM rust:bullseye

WORKDIR /usr/src/discord_gm_bot

COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["discord_gm_bot"]