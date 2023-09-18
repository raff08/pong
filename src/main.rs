use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};

const PADDING: f32 = 40.0;  // Espaçamento entre a borda e as raquetes
const MIDDLE_LINE_W: f32 = 2.0;  // Largura da linha no meio da tela
const ALTURA_RAQUETE: f32 = 100.0;  // Altura das raquetes
const LARGURA_RAQUETE: f32 = 20.0;  // Largura das raquetes
const LARGURA_RAQUETE_MEIO: f32 = LARGURA_RAQUETE * 0.5;  // Metade da largura das raquetes
const ALTURA_RAQUETE_MEIO: f32 = ALTURA_RAQUETE * 0.5;  // Metade da altura das raquetes
const TAMANHO_BOLA: f32 = 30.0;  // Tamanho da bola
const TAMANHO_BOLA_MEIO: f32 = TAMANHO_BOLA * 0.5;  // Metade do tamanho da bola
const VELOCIDADE_JOGADOR: f32 = 600.0;  // Velocidade de movimento dos jogadores
const VELOCIDADE_BOLA: f32 = 500.0;  // Velocidade da bola

// Função auxiliar para limitar um valor dentro de um intervalo
fn clamp(value: &mut f32, baixo: f32, alto: f32) {
    if *value < baixo {
        *value = baixo;
    } else if *value > alto {
        *value = alto;
    }
}

// Função auxiliar para mover uma raquete com base em uma tecla pressionada
fn mover_raquete(pos: &mut na::Point2<f32>, tecla: KeyCode, direcao_y: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    let altura_tela = graphics::drawable_size(ctx).1;
    if keyboard::is_key_pressed(ctx, tecla) {
        pos.y += direcao_y * VELOCIDADE_JOGADOR * dt;
    }
    clamp(
        &mut pos.y,
        ALTURA_RAQUETE_MEIO,
        altura_tela - ALTURA_RAQUETE_MEIO,
    );
}

// Função auxiliar para randomizar um vetor com base em probabilidades
fn randomizar_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

// Estrutura que representa o estado do jogo
struct EstadoJogo {
    posicao_jogador1: na::Point2<f32>,
    posicao_jogador2: na::Point2<f32>,
    posicao_bola: na::Point2<f32>,
    velocidade_bola: na::Vector2<f32>,
    placar_jogador1: i32,
    placar_jogador2: i32,
}

impl EstadoJogo {
    // Construtor para criar um novo estado de jogo
    pub fn novo(ctx: &mut Context) -> Self {
        let (largura_tela, altura_tela) = graphics::drawable_size(ctx);
        let (largura_tela_meio, altura_tela_meio) = (largura_tela * 0.5, altura_tela * 0.5);

        let mut velocidade_bola = na::Vector2::new(0.0, 0.0);
        randomizar_vec(&mut velocidade_bola, VELOCIDADE_BOLA, VELOCIDADE_BOLA);

        EstadoJogo {
            posicao_jogador1: na::Point2::new(LARGURA_RAQUETE_MEIO + PADDING, altura_tela_meio),
            posicao_jogador2: na::Point2::new(largura_tela - LARGURA_RAQUETE_MEIO - PADDING, altura_tela_meio),
            posicao_bola: na::Point2::new(largura_tela_meio, altura_tela_meio),
            velocidade_bola,
            placar_jogador1: 0,
            placar_jogador2: 0,
        }
    }
}

impl event::EventHandler for EstadoJogo {
    // Função para atualizar o estado do jogo
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (largura_tela, altura_tela) = graphics::drawable_size(ctx);
        mover_raquete(&mut self.posicao_jogador1, KeyCode::W, -1.0, ctx);
        mover_raquete(&mut self.posicao_jogador1, KeyCode::S, 1.0, ctx);
        mover_raquete(&mut self.posicao_jogador2, KeyCode::Up, -1.0, ctx);
        mover_raquete(&mut self.posicao_jogador2, KeyCode::Down, 1.0, ctx);

        self.posicao_bola += self.velocidade_bola * dt;

        // Lógica para reiniciar a bola e atualizar o placar quando a bola sai da tela
        if self.posicao_bola.x < 0.0 {
            self.posicao_bola.x = largura_tela * 0.5;
            self.posicao_bola.y = altura_tela * 0.5;
            randomizar_vec(&mut self.velocidade_bola, VELOCIDADE_BOLA, VELOCIDADE_BOLA);
            self.placar_jogador2 += 1;
        }
        if self.posicao_bola.x > largura_tela {
            self.posicao_bola.x = largura_tela * 0.5;
            self.posicao_bola.y = altura_tela * 0.5;
            randomizar_vec(&mut self.velocidade_bola, VELOCIDADE_BOLA, VELOCIDADE_BOLA);
            self.placar_jogador1 += 1;
        }

        // Lógica para quicar a bola nas extremidades superior e inferior da tela
        if self.posicao_bola.y < TAMANHO_BOLA_MEIO {
            self.posicao_bola.y = TAMANHO_BOLA_MEIO;
            self.velocidade_bola
            .y = self.velocidade_bola.y.abs();  // Inverte a direção da bola

        } else if self.posicao_bola.y > altura_tela - TAMANHO_BOLA_MEIO {
            self.posicao_bola.y = altura_tela - TAMANHO_BOLA_MEIO;
            self.velocidade_bola.y = -self.velocidade_bola.y.abs();
        }

        // Lógica para verificar se a bola colidiu com as raquetes dos jogadores
        let colidiu_com_jogador1 = self.posicao_bola.x - TAMANHO_BOLA_MEIO
            < self.posicao_jogador1.x + LARGURA_RAQUETE_MEIO
            && self.posicao_bola.x + TAMANHO_BOLA_MEIO > self.posicao_jogador1.x - LARGURA_RAQUETE_MEIO
            && self.posicao_bola.y - TAMANHO_BOLA_MEIO < self.posicao_jogador1.y + ALTURA_RAQUETE_MEIO
            && self.posicao_bola.y + TAMANHO_BOLA_MEIO > self.posicao_jogador1.y - ALTURA_RAQUETE_MEIO;

        if colidiu_com_jogador1 {
            self.velocidade_bola.x = self.velocidade_bola.x.abs();
        }

        let colidiu_com_jogador2 = self.posicao_bola.x - TAMANHO_BOLA_MEIO
            < self.posicao_jogador2.x + LARGURA_RAQUETE_MEIO
            && self.posicao_bola.x + TAMANHO_BOLA_MEIO > self.posicao_jogador2.x - LARGURA_RAQUETE_MEIO
            && self.posicao_bola.y - TAMANHO_BOLA_MEIO < self.posicao_jogador2.y + ALTURA_RAQUETE_MEIO
            && self.posicao_bola.y + TAMANHO_BOLA_MEIO > self.posicao_jogador2.y - ALTURA_RAQUETE_MEIO;

        if colidiu_com_jogador2 {
            self.velocidade_bola.x = -self.velocidade_bola.x.abs();
        }
        Ok(())
    }

    // Função para desenhar o estado atual do jogo
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // Criação dos objetos gráficos para as raquetes, a bola e a linha no meio da tela
        let retangulo_raquete = graphics::Rect::new(
            -LARGURA_RAQUETE_MEIO,
            -ALTURA_RAQUETE_MEIO,
            LARGURA_RAQUETE,
            ALTURA_RAQUETE,
        );
        let malha_raquete = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            retangulo_raquete,
            graphics::WHITE,
        )?;

        let retângulo_bola = graphics::Rect::new(-TAMANHO_BOLA_MEIO, -TAMANHO_BOLA_MEIO, TAMANHO_BOLA, TAMANHO_BOLA);
        let malha_bola = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            retângulo_bola,
            graphics::WHITE,
        )?;

        let altura_tela = graphics::drawable_size(ctx).1;
        let retângulo_meio = graphics::Rect::new(-MIDDLE_LINE_W * 0.5, 0.0, MIDDLE_LINE_W, altura_tela);
        let malha_meio = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            retângulo_meio,
            graphics::WHITE,
        )?;

        let mut parâmetro_desenho = graphics::DrawParam::default();

        let meio_tela_x = graphics::drawable_size(ctx).0 * 0.5;
        parâmetro_desenho.dest = [meio_tela_x, 0.0].into();
        graphics::draw(ctx, &malha_meio, parâmetro_desenho)?;

        parâmetro_desenho.dest = self.posicao_jogador1.into();
        graphics::draw(ctx, &malha_raquete, parâmetro_desenho)?;

        parâmetro_desenho.dest = self.posicao_jogador2.into();
        graphics::draw(ctx, &malha_raquete, parâmetro_desenho)?;

        parâmetro_desenho.dest = self.posicao_bola.into();
        graphics::draw(ctx, &malha_bola, parâmetro_desenho)?;

        // Exibe o placar na tela
        let texto_placar = graphics::Text::new(format!(
            "{}         {}",
            self.placar_jogador1, self.placar_jogador2
        ));
        let largura_tela = graphics::drawable_size(ctx).0;
        let largura_tela_meio = largura_tela * 0.5;

        let mut posição_placar = na::Point2::new(largura_tela_meio, 40.0);
        let (largura_texto_placar, altura_texto_placar) = texto_placar.dimensions(ctx);
        posição_placar -= na::Vector2::new(largura_texto_placar as f32 * 0.5, altura_texto_placar as f32 * 0.5);
        parâmetro_desenho.dest = posição_placar.into();

        graphics::draw(ctx, &texto_placar, parâmetro_desenho)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let construtor_contexto = ggez::ContextBuilder::new("pong", "Rafael");
    let (mut ctx, mut event_loop) = construtor_contexto.build()?;
    graphics::set_window_title(&ctx, "Rusty Pong");
    let mut estado_jogo = EstadoJogo::novo(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut estado_jogo)
}
