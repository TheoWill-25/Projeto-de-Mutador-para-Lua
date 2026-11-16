mod mutador;
mod parser;
mod scanner;

use eframe::egui;
use egui::{Color32, RichText};
use mutador::{symbol_mutator, Mutation};
use parser::parse_lua;
use scanner::symbol_scanner;
use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread;

// Paleta: ~60% branco, ~25% cinza/prata, ~15% azul
const BRANCO: Color32 = Color32::from_rgb(255, 255, 255);
const CINZA: Color32 = Color32::from_rgb(229, 231, 235);
const CINZA_CLARO: Color32 = Color32::from_rgb(243, 244, 246);
const BORDA: Color32 = Color32::from_rgb(209, 213, 219);
const AZUL: Color32 = Color32::from_rgb(37, 99, 235);
const AZUL_CLARO: Color32 = Color32::from_rgb(219, 234, 254);
const TEXTO: Color32 = Color32::from_rgb(31, 41, 55);
const SUAVE: Color32 = Color32::from_rgb(107, 114, 128);
const VERMELHO: Color32 = Color32::from_rgb(190, 18, 60);

struct Resultado {
    total: usize,
    sobreviventes: Vec<Mutation>,
    erros: usize,
}

/// O mesmo fluxo do main antigo, só que recebendo os caminhos da interface.
fn executar(path_code: String, path_suite: String) -> Result<Resultado, String> {
    let codigo = fs::read_to_string(&path_code)
        .map_err(|e| format!("Erro ao ler o código: {e}"))?;

    // Roda na pasta do código (o relatorio.json também sai ali).
    if let Some(dir) = Path::new(&path_code).parent() {
        let _ = std::env::set_current_dir(dir);
    }

    let ast = parse_lua(&codigo);
    let json = serde_json::to_string_pretty(&ast)
        .expect("Erro ao converter AST para JSON");

    fs::write("AST.json", &json)
        .expect("Erro ao escrever relatório");
        
    let symbols = symbol_scanner(&ast);
    let mutation = symbol_mutator(&codigo, symbols, path_code, path_suite);

    let sobreviventes: Vec<Mutation> = mutation
        .iter()
        .filter(|m| m.failed_tests.is_empty() && !m.error)
        .cloned()
        .collect();

    let erros = mutation.iter().filter(|m| m.error).count();

    let json = serde_json::to_string_pretty(&sobreviventes)
        .map_err(|e| format!("Erro ao gerar JSON: {e}"))?;
    fs::write("relatorio.json", json)
        .map_err(|e| format!("Erro ao escrever relatório: {e}"))?;

    Ok(Resultado {
        total: mutation.len(),
        sobreviventes,
        erros,
    })
}

#[derive(Default)]
struct MyApp {
    path_code: Option<String>,
    path_suite: Option<String>,
    rx: Option<Receiver<Result<Resultado, String>>>,
    running: bool,
    resultado: Option<Resultado>,
    erro: Option<String>,
}

fn configurar_tema(ctx: &egui::Context) {
    let mut v = egui::Visuals::light();
    v.panel_fill = BRANCO;
    v.window_fill = BRANCO;
    v.override_text_color = Some(TEXTO);
    v.widgets.inactive.bg_fill = CINZA;
    v.widgets.inactive.weak_bg_fill = CINZA;
    v.widgets.hovered.bg_fill = AZUL_CLARO;
    v.widgets.hovered.weak_bg_fill = AZUL_CLARO;
    v.widgets.active.bg_fill = AZUL;
    v.widgets.active.weak_bg_fill = AZUL;
    v.widgets.noninteractive.bg_stroke.color = BORDA;
    ctx.set_visuals(v);
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([980.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lua Mutation Tester",
        options,
        Box::new(|cc| {
            configurar_tema(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

fn nome(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string())
}

fn cartao(ui: &mut egui::Ui, titulo: &str, valor: usize, destaque: bool) {
    let (fundo, cor, cor_titulo) = if destaque {
        (AZUL, BRANCO, BRANCO)
    } else {
        (CINZA_CLARO, TEXTO, SUAVE)
    };

    egui::Frame::new()
        .fill(fundo)
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(titulo).color(cor_titulo).size(12.0));
                ui.label(RichText::new(valor.to_string()).color(cor).size(24.0).strong());
            });
        });
}

impl MyApp {
    fn iniciar(&mut self) {
        let (Some(code), Some(suite)) = (self.path_code.clone(), self.path_suite.clone()) else {
            return;
        };

        let (tx, rx) = channel();
        self.rx = Some(rx);
        self.running = true;
        self.resultado = None;
        self.erro = None;

        // Em outra thread para a janela não travar durante os testes.
        thread::spawn(move || {
            let _ = tx.send(executar(code, suite));
        });
    }

    fn parar(&mut self) {
        self.running = false;
        self.rx = None;
    }

    fn checar_thread(&mut self) {
        let msg = self.rx.as_ref().map(|rx| rx.try_recv());

        match msg {
            Some(Ok(Ok(r))) => {
                self.resultado = Some(r);
                self.parar();
            }
            Some(Ok(Err(e))) => {
                self.erro = Some(e);
                self.parar();
            }
            // A thread caiu (ex.: Lua inválido faz o parser dar panic)
            Some(Err(TryRecvError::Disconnected)) => {
                self.erro = Some("A execução falhou. Veja o terminal para detalhes.".to_string());
                self.parar();
            }
            _ => {}
        }
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);

        ui.label(RichText::new("Código Lua").strong());
        if ui.add_enabled(!self.running, egui::Button::new("Selecionar código")).clicked() {
            if let Some(p) = rfd::FileDialog::new().add_filter("Lua", &["lua"]).pick_file() {
                self.path_code = Some(p.display().to_string());
            }
        }
        match &self.path_code {
            Some(p) => ui.label(RichText::new(nome(p)).color(AZUL)),
            None => ui.label(RichText::new("Nenhum selecionado").color(SUAVE)),
        };

        ui.add_space(18.0);

        ui.label(RichText::new("Suíte de testes").strong());
        if ui.add_enabled(!self.running, egui::Button::new("Selecionar suíte")).clicked() {
            if let Some(p) = rfd::FileDialog::new().add_filter("Lua", &["lua"]).pick_file() {
                self.path_suite = Some(p.display().to_string());
            }
        }
        match &self.path_suite {
            Some(p) => ui.label(RichText::new(nome(p)).color(AZUL)),
            None => ui.label(RichText::new("Nenhuma selecionada").color(SUAVE)),
        };

        ui.add_space(24.0);
        ui.separator();
        ui.add_space(12.0);

        let pode = self.path_code.is_some() && self.path_suite.is_some() && !self.running;
        let texto = if self.running { "Executando..." } else { "Executar mutador" };

        let botao = egui::Button::new(RichText::new(texto).color(BRANCO).strong())
            .fill(AZUL)
            .corner_radius(6)
            .min_size(egui::vec2(ui.available_width(), 40.0));

        if ui.add_enabled(pode, botao).clicked() {
            self.iniciar();
        }

        if self.running {
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(RichText::new("Rodando os testes...").color(SUAVE));
            });
        }
    }

    fn resultados(&mut self, ui: &mut egui::Ui) {
        if let Some(e) = &self.erro {
            ui.label(RichText::new(format!("Erro: {e}")).color(VERMELHO));
            return;
        }

        let Some(r) = &self.resultado else {
            ui.add_space(80.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Selecione o código e a suíte, depois clique em \"Executar mutador\".")
                        .color(SUAVE)
                        .size(16.0),
                );
            });
            return;
        };

        ui.horizontal(|ui| {
            cartao(ui, "Mutações totais", r.total, false);
            cartao(ui, "Sobreviventes", r.sobreviventes.len(), true);
            cartao(ui, "Com erro", r.erros, false);
        });

        ui.add_space(16.0);
        ui.label(RichText::new("Mutações sobreviventes").size(18.0).strong());
        ui.label(RichText::new("Relatório salvo em relatorio.json").color(SUAVE).size(12.0));
        ui.add_space(8.0);

        if r.sobreviventes.is_empty() {
            ui.label("Nenhuma mutação sobreviveu. Sua suíte pegou todas!");
            return;
        }

        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for m in &r.sobreviventes {
                egui::Frame::new()
                    .fill(CINZA_CLARO)
                    .stroke(egui::Stroke::new(1.0, BORDA))
                    .corner_radius(8)
                    .inner_margin(12)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(
                            RichText::new(format!("Linha {} · {}", m.candidate.line, m.candidate.kind))
                                .color(AZUL)
                                .strong(),
                        );
                        ui.monospace(format!("- {}", m.og_line.trim()));
                        ui.monospace(format!("+ {}", m.mod_line.trim()));
                    });
                ui.add_space(6.0);
            }
        });
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.checar_thread();
        if self.running {
            ui.ctx().request_repaint();
        }

        // Faixa azul
        egui::Panel::top("header")
            .exact_size(56.0)
            .frame(egui::Frame::new().fill(AZUL).inner_margin(egui::Margin::symmetric(20, 0)))
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(RichText::new("Lua Mutation Tester").color(BRANCO).size(22.0).strong());
                });
            });

        // Barra lateral cinza (25%)
        let largura = ui.available_width() * 0.25;
        egui::Panel::left("sidebar")
            .resizable(false)
            .exact_size(largura)
            .frame(egui::Frame::new().fill(CINZA).stroke(egui::Stroke::new(1.0, BORDA)).inner_margin(16))
            .show(ui, |ui| self.sidebar(ui));

        // Área branca
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BRANCO).inner_margin(20))
            .show(ui, |ui| self.resultados(ui));
    }
}