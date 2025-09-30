import SwiftUI
import UIKit
import PhotosUI

struct ContentView: View {
    @State private var output: String = "Toque em um exemplo para chamar o FFI"
    @State private var comparePaths: (baseline: String, input: String)?
    @State private var searchPaths: (parent: String, child: String)?
    @State private var locatePaths: (container: String, main: String, relative: String)?
    @State private var baselineImage: UIImage?
    @State private var inputImage: UIImage?
    @State private var diffImage: UIImage?
    @State private var showBaselinePicker = false
    @State private var showInputPicker = false
    @State private var obtainedSimilarity: Double?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                Group {
                    Text("Visualizar e Comparar")
                        .font(.title2).bold()
                    HStack(spacing: 12) {
                        VStack {
                            ZStack {
                                Rectangle().fill(Color(UIColor.secondarySystemBackground))
                                if let img = baselineImage { Image(uiImage: img).resizable().scaledToFit() }
                                else { Text("Baseline").foregroundColor(.secondary) }
                            }
                            .frame(height: 140)
                            Button("Escolher Baseline") { showBaselinePicker = true }
                                .buttonStyle(.bordered)
                        }
                        VStack {
                            ZStack {
                                Rectangle().fill(Color(UIColor.secondarySystemBackground))
                                if let img = inputImage { Image(uiImage: img).resizable().scaledToFit() }
                                else { Text("Input").foregroundColor(.secondary) }
                            }
                            .frame(height: 140)
                            Button("Escolher Input") { showInputPicker = true }
                                .buttonStyle(.bordered)
                        }
                    }
                    if let d = diffImage {
                        VStack(alignment: .leading) {
                            Text("Diff (visual)")
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                            Image(uiImage: d).resizable().scaledToFit().frame(height: 140)
                        }
                    }
                    Button("Comparar selecionadas") { runCompareSelected() }
                        .buttonStyle(.borderedProminent)
                }

                Group {
                    Text("Exemplos Reais (gera imagens de exemplo em /tmp)")
                        .font(.headline)
                    Button("Preparar imagens de exemplo") {
                        prepareExamples()
                    }
                    .buttonStyle(.bordered)
                    .accessibilityIdentifier("prepare-button")
                }

                Divider()

                Group {
                    Text("Compare")
                        .font(.title3).bold()
                    Button("vt_compare_images") {
                        runCompare()
                    }
                    .buttonStyle(.borderedProminent)
                    .accessibilityIdentifier("compare-button")
                    if let p = comparePaths {
                        Text("baseline: \(p.baseline)\ninput: \(p.input)")
                            .font(.footnote)
                            .foregroundColor(.secondary)
                    }
                }

                Group {
                    Text("Search")
                        .font(.title3).bold()
                    Button("vt_flex_search") {
                        runSearch()
                    }
                    .buttonStyle(.bordered)
                    .accessibilityIdentifier("search-button")
                    if let p = searchPaths {
                        Text("parent: \(p.parent)\nchild: \(p.child)")
                            .font(.footnote)
                            .foregroundColor(.secondary)
                    }
                }

                Group {
                    Text("Locate")
                        .font(.title3).bold()
                    Button("vt_flex_locate") {
                        runLocate()
                    }
                    .buttonStyle(.bordered)
                    .accessibilityIdentifier("locate-button")
                    if let p = locatePaths {
                        Text("container: \(p.container)\nmain: \(p.main)\nrelative: \(p.relative)")
                            .font(.footnote)
                            .foregroundColor(.secondary)
                    }
                }

                Group {
                    Text("Saída JSON")
                        .font(.headline)
                    if let sim = obtainedSimilarity {
                        Text(String(format: "Similaridade: %.2f%%", sim))
                            .font(.headline)
                            .foregroundColor(.accentColor)
                    }
                    Text(output)
                        .font(.system(.body, design: .monospaced))
                        .padding(8)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(Color(UIColor.secondarySystemBackground))
                        .cornerRadius(8)
                        .accessibilityIdentifier("json-output")
                }
            }
            .padding()
        }
        .onAppear { prepareExamples() }
        .sheet(isPresented: $showBaselinePicker) {
            PhotoPickerView { image in
                if let image = image, let path = saveUIImageToTemp(image, name: "user_baseline.png") {
                    baselineImage = image
                    updateComparePaths(baseline: path, input: comparePaths?.input)
                    diffImage = nil
                    obtainedSimilarity = nil
                }
            }
        }
        .sheet(isPresented: $showInputPicker) {
            PhotoPickerView { image in
                if let image = image, let path = saveUIImageToTemp(image, name: "user_input.png") {
                    inputImage = image
                    updateComparePaths(baseline: comparePaths?.baseline, input: path)
                    diffImage = nil
                    obtainedSimilarity = nil
                }
            }
        }
    }

    // MARK: - FFI Calls

    private func runCompare() {
        let paths = comparePaths ?? prepareExamples()
        let minSim: Int32 = 95
        let noise: Int32 = 10
        let excluded = "[]" // Sem áreas excluídas
        let meta = "{\"testName\":\"iOS-Demo-Compare\"}"

        paths.baseline.withCString { b in
            paths.input.withCString { i in
                excluded.withCString { ex in
                    meta.withCString { me in
                        if let res = vt_compare_images(b, i, minSim, noise, ex, me) {
                            let json = String(cString: res)
                            vt_free_string(res)
                            DispatchQueue.main.async {
                                self.output = pretty(json)
                                self.obtainedSimilarity = extractSimilarity(json)
                                if let path = extractDiffPath(json), let img = UIImage(contentsOfFile: path) {
                                    self.diffImage = img
                                } else {
                                    self.diffImage = computeDiff()
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    private func runCompareSelected() {
        guard let p = comparePaths else { return }
        let minSim: Int32 = 95
        let noise: Int32 = 10
        let excluded = "[]"
        let meta = "{\"testName\":\"iOS-Demo-Compare-Selected\"}"
        p.baseline.withCString { b in
            p.input.withCString { i in
                excluded.withCString { ex in
                    meta.withCString { me in
                        if let res = vt_compare_images(b, i, minSim, noise, ex, me) {
                            let json = String(cString: res)
                            vt_free_string(res)
                            DispatchQueue.main.async {
                                self.output = pretty(json)
                                self.obtainedSimilarity = extractSimilarity(json)
                                if let path = extractDiffPath(json), let img = UIImage(contentsOfFile: path) {
                                    self.diffImage = img
                                } else {
                                    self.diffImage = computeDiff()
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    private func runSearch() {
        guard let paths = searchPaths ?? optionalPrepareSearch() else { return }
        let meta = "{\"testName\":\"iOS-Demo-Search\"}"
        paths.parent.withCString { p in
            paths.child.withCString { c in
                meta.withCString { me in
                    if let res = vt_flex_search(p, c, me) {
                        let json = String(cString: res)
                        vt_free_string(res)
                        DispatchQueue.main.async { self.output = pretty(json) }
                    }
                }
            }
        }
    }

    private func runLocate() {
        guard let paths = locatePaths ?? optionalPrepareLocate() else { return }
        let meta = "{\"testName\":\"iOS-Demo-Locate\"}"
        paths.container.withCString { co in
            paths.main.withCString { ma in
                paths.relative.withCString { re in
                    meta.withCString { me in
                        if let res = vt_flex_locate(co, ma, re, me) {
                            let json = String(cString: res)
                            vt_free_string(res)
                            DispatchQueue.main.async { self.output = pretty(json) }
                        }
                    }
                }
            }
        }
    }

    // MARK: - Sample data

    @discardableResult
    private func prepareExamples() -> (baseline: String, input: String) {
        let dir = NSTemporaryDirectory()
        let baselinePath = (dir as NSString).appendingPathComponent("baseline.png")
        let inputPath = (dir as NSString).appendingPathComponent("input.png")

        // baseline: fundo branco com círculo vermelho central
        let baselineImage = drawImage(size: CGSize(width: 200, height: 200)) { ctx, size in
            UIColor.white.setFill(); ctx.fill(CGRect(origin: .zero, size: size))
            UIColor.red.setFill(); UIBezierPath(ovalIn: CGRect(x: 50, y: 50, width: 100, height: 100)).fill()
        }
        // input: círculo levemente deslocado
        let inputImage = drawImage(size: CGSize(width: 200, height: 200)) { ctx, size in
            UIColor.white.setFill(); ctx.fill(CGRect(origin: .zero, size: size))
            UIColor.red.setFill(); UIBezierPath(ovalIn: CGRect(x: 60, y: 50, width: 100, height: 100)).fill()
        }

        if let data = baselineImage.pngData() { try? data.write(to: URL(fileURLWithPath: baselinePath), options: .atomic) }
        if let data = inputImage.pngData() { try? data.write(to: URL(fileURLWithPath: inputPath), options: .atomic) }

        // prepare search/locate too
        _ = optionalPrepareSearch()
        _ = optionalPrepareLocate()

        self.comparePaths = (baselinePath, inputPath)
        self.baselineImage = baselineImage
        self.inputImage = inputImage
        self.diffImage = computeDiff()
        return (baselinePath, inputPath)
    }

    private func optionalPrepareSearch() -> (parent: String, child: String)? {
        let dir = NSTemporaryDirectory()
        let parentPath = (dir as NSString).appendingPathComponent("parent.png")
        let childPath = (dir as NSString).appendingPathComponent("child.png")

        let parent = drawImage(size: CGSize(width: 240, height: 180)) { ctx, size in
            UIColor.white.setFill(); ctx.fill(CGRect(origin: .zero, size: size))
            UIColor.blue.setFill(); UIBezierPath(rect: CGRect(x: 140, y: 60, width: 60, height: 60)).fill()
        }
        let child = drawImage(size: CGSize(width: 60, height: 60)) { ctx, _ in
            UIColor.blue.setFill(); UIBezierPath(rect: CGRect(x: 0, y: 0, width: 60, height: 60)).fill()
        }

        if let data = parent.pngData() { try? data.write(to: URL(fileURLWithPath: parentPath), options: .atomic) }
        if let data = child.pngData() { try? data.write(to: URL(fileURLWithPath: childPath), options: .atomic) }

        let out = (parentPath, childPath)
        self.searchPaths = out
        return out
    }

    private func optionalPrepareLocate() -> (container: String, main: String, relative: String)? {
        let dir = NSTemporaryDirectory()
        let containerPath = (dir as NSString).appendingPathComponent("container.png")
        let mainPath = (dir as NSString).appendingPathComponent("main.png")
        let relativePath = (dir as NSString).appendingPathComponent("relative.png")

        let container = drawImage(size: CGSize(width: 300, height: 200)) { ctx, size in
            UIColor.white.setFill(); ctx.fill(CGRect(origin: .zero, size: size))
            UIColor.green.setFill(); UIBezierPath(rect: CGRect(x: 80, y: 70, width: 60, height: 60)).fill()
            UIColor.orange.setFill(); UIBezierPath(rect: CGRect(x: 180, y: 90, width: 60, height: 40)).fill()
        }
        let main = drawImage(size: CGSize(width: 60, height: 60)) { ctx, _ in
            UIColor.green.setFill(); UIBezierPath(rect: CGRect(x: 0, y: 0, width: 60, height: 60)).fill()
        }
        let relative = drawImage(size: CGSize(width: 60, height: 40)) { ctx, _ in
            UIColor.orange.setFill(); UIBezierPath(rect: CGRect(x: 0, y: 0, width: 60, height: 40)).fill()
        }

        if let data = container.pngData() { try? data.write(to: URL(fileURLWithPath: containerPath), options: .atomic) }
        if let data = main.pngData() { try? data.write(to: URL(fileURLWithPath: mainPath), options: .atomic) }
        if let data = relative.pngData() { try? data.write(to: URL(fileURLWithPath: relativePath), options: .atomic) }

        let out = (containerPath, mainPath, relativePath)
        self.locatePaths = out
        return out
    }

    // MARK: - Utils

    private func drawImage(size: CGSize, draw: (CGContext, CGSize) -> Void) -> UIImage {
        let format = UIGraphicsImageRendererFormat()
        format.scale = 1
        format.opaque = true
        let renderer = UIGraphicsImageRenderer(size: size, format: format)
        let img = renderer.image { ctx in
            draw(ctx.cgContext, size)
        }
        return img
    }

    private func pretty(_ json: String) -> String {
        guard let data = json.data(using: .utf8),
              let obj = try? JSONSerialization.jsonObject(with: data),
              let prettyData = try? JSONSerialization.data(withJSONObject: obj, options: [.prettyPrinted])
        else { return json }
        return String(data: prettyData, encoding: .utf8) ?? json
    }

    private func extractSimilarity(_ json: String) -> Double? {
        guard let data = json.data(using: .utf8),
              let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let sim = obj["obtainedSimilarity"] as? Double else { return nil }
        return sim
    }

    private func extractDiffPath(_ json: String) -> String? {
        guard let data = json.data(using: .utf8),
              let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else { return nil }
        if let s = obj["resultImageRef"] as? String, !s.isEmpty { return s }
        return nil
    }

    private func saveUIImageToTemp(_ image: UIImage, name: String) -> String? {
        let path = (NSTemporaryDirectory() as NSString).appendingPathComponent(name)
        if let data = image.pngData() {
            do {
                try data.write(to: URL(fileURLWithPath: path), options: .atomic)
                return path
            } catch {
                return nil
            }
        }
        return nil
    }

    private func updateComparePaths(baseline: String?, input: String?) {
        let b = baseline ?? comparePaths?.baseline
        let i = input ?? comparePaths?.input
        if let b = b, let i = i {
            comparePaths = (b, i)
        }
    }

    private func computeDiff() -> UIImage? {
        guard let a = baselineImage, let b = inputImage else { return nil }
        let size = a.size
        let bResized = resizeImage(b, to: size)
        guard let ciA = CIImage(image: a), let ciB = CIImage(image: bResized),
              let filter = CIFilter(name: "CIDifferenceBlendMode") else { return nil }
        filter.setValue(ciA, forKey: kCIInputBackgroundImageKey)
        filter.setValue(ciB, forKey: kCIInputImageKey)
        guard let out = filter.outputImage else { return nil }
        let ctx = CIContext()
        if let cg = ctx.createCGImage(out, from: out.extent) {
            return UIImage(cgImage: cg, scale: 1, orientation: .up)
        }
        return nil
    }

    private func resizeImage(_ image: UIImage, to size: CGSize) -> UIImage {
        let format = UIGraphicsImageRendererFormat()
        format.scale = 1
        format.opaque = true
        let renderer = UIGraphicsImageRenderer(size: size, format: format)
        return renderer.image { _ in
            image.draw(in: CGRect(origin: .zero, size: size))
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
