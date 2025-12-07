FROM rust:bullseye

WORKDIR /usr/src/discord_gm_bot

ADD src/ /usr/src/discord_gm_bot/src
COPY ./Cargo.toml .

RUN ls -lrta 


RUN cargo install --path .

EXPOSE 3000

CMD ["discord_gm_bot"]