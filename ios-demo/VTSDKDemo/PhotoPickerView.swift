import SwiftUI
import PhotosUI

struct PhotoPickerView: UIViewControllerRepresentable {
    var onImagePicked: (UIImage?) -> Void

    func makeCoordinator() -> Coordinator { Coordinator(onPicked: onImagePicked) }

    func makeUIViewController(context: Context) -> PHPickerViewController {
        var config = PHPickerConfiguration(photoLibrary: .shared())
        config.selectionLimit = 1
        config.filter = .images
        let vc = PHPickerViewController(configuration: config)
        vc.delegate = context.coordinator
        return vc
    }

    func updateUIViewController(_ uiViewController: PHPickerViewController, context: Context) {}

    final class Coordinator: NSObject, PHPickerViewControllerDelegate {
        let onPicked: (UIImage?) -> Void
        init(onPicked: @escaping (UIImage?) -> Void) { self.onPicked = onPicked }
        func picker(_ picker: PHPickerViewController, didFinishPicking results: [PHPickerResult]) {
            picker.dismiss(animated: true)
            guard let item = results.first else { return onPicked(nil) }
            if item.itemProvider.canLoadObject(ofClass: UIImage.self) {
                item.itemProvider.loadObject(ofClass: UIImage.self) { obj, _ in
                    DispatchQueue.main.async { self.onPicked(obj as? UIImage) }
                }
            } else {
                onPicked(nil)
            }
        }
    }
}

