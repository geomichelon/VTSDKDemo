# iOS Demo

Projeto SwiftUI de exemplo que consome o SDK via FFI usando o `VTSDK.xcframework` gerado em `dist/` e o header `ffi/include/vt_sdk.h`.

## Como abrir e rodar

1) Gere o XCFramework (já fizemos): `dist/VTSDK.xcframework`.
2) Abra `ios-demo/VTSDKDemo.xcodeproj` no Xcode.
3) No target `VTSDKDemo`, ajuste o `Team` em Signing & Capabilities.
4) Selecione um simulador ou dispositivo e rode.

O botão "Run FFI" chama `vt_compare_images` e mostra o JSON retornado.

Exemplos incluídos
- O app gera imagens de exemplo no diretório temporário e executa:
  - `vt_compare_images` (baseline vs. input)
  - `vt_flex_search` (child dentro de parent)
  - `vt_flex_locate` (main e relative dentro de container)
  Os JSONs são exibidos formatados na tela.

Visualização e comparação
- É possível escolher duas imagens da Fototeca (botões “Escolher Baseline” e “Escolher Input”).
- As imagens são mostradas lado a lado; toque “Comparar selecionadas” para rodar `vt_compare_images` com esses arquivos.
- A similaridade retornada aparece no JSON abaixo.

## UI Tests (modelo)

- Scheme extra de testes: `ios-demo/VTSDKDemo.xcodeproj/xcshareddata/xcschemes/VTSDKDemo-UI.xcscheme` (Debug)
- Adicione um target iOS UI Testing no Xcode chamado `VTSDKDemoUITests` e inclua o arquivo `ios-demo/VTSDKDemoUITests/VTSDKDemoUITests.swift`.
- Rode os testes com a scheme `VTSDKDemo-UI`; o teste toca a UI do sample e valida o JSON mostrado.

Estrutura principal:
- Projeto: `ios-demo/VTSDKDemo.xcodeproj`
- Código fonte: `ios-demo/VTSDKDemo/`
- Bridging header: `ios-demo/VTSDKDemo/VTSDKDemo-Bridging-Header.h`
- Assets: `ios-demo/VTSDKDemo/Assets.xcassets`
