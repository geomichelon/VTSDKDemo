package com.example.vtsdkdemo

import android.content.ContentResolver
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Bundle
import android.provider.OpenableColumns
import android.widget.Button
import android.widget.ImageView
import android.widget.TextView
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import java.io.File

class MainActivity : AppCompatActivity() {
    private lateinit var imgBaseline: ImageView
    private lateinit var imgInput: ImageView
    private lateinit var imgDiff: ImageView
    private lateinit var txtJson: TextView
    private lateinit var txtSim: TextView

    private var baselinePath: String? = null
    private var inputPath: String? = null

    private val pickBaseline = registerForActivityResult(ActivityResultContracts.OpenDocument()) { uri: Uri? ->
        uri?.let {
            baselinePath = copyToCache(it, "baseline.png")
            baselinePath?.let { p -> imgBaseline.setImageBitmap(BitmapFactory.decodeFile(p)) }
            imgDiff.setImageDrawable(null)
            txtSim.text = "Similarity: -"
        }
    }

    private val pickInput = registerForActivityResult(ActivityResultContracts.OpenDocument()) { uri: Uri? ->
        uri?.let {
            inputPath = copyToCache(it, "input.png")
            inputPath?.let { p -> imgInput.setImageBitmap(BitmapFactory.decodeFile(p)) }
            imgDiff.setImageDrawable(null)
            txtSim.text = "Similarity: -"
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        imgBaseline = findViewById(R.id.imageBaseline)
        imgInput = findViewById(R.id.imageInput)
        imgDiff = findViewById(R.id.imageDiff)
        txtJson = findViewById(R.id.textJson)
        txtSim = findViewById(R.id.textSimilarity)

        findViewById<Button>(R.id.btnPickBaseline).setOnClickListener {
            pickBaseline.launch(arrayOf("image/*"))
        }
        findViewById<Button>(R.id.btnPickInput).setOnClickListener {
            pickInput.launch(arrayOf("image/*"))
        }
        findViewById<Button>(R.id.btnCompare).setOnClickListener { runCompare() }
    }

    private fun runCompare() {
        val b = baselinePath ?: return
        val i = inputPath ?: return
        val json = VtSdkFFI.vtCompareImages(b, i, 95, 20, "[]", "{\"testName\":\"Android-Demo\"}")
        txtJson.text = pretty(json)
        extractSimilarity(json)?.let { sim -> txtSim.text = String.format("Similarity: %.2f%%", sim) }
        extractDiffPath(json)?.let { path ->
            val bmp = BitmapFactory.decodeFile(path)
            if (bmp != null) imgDiff.setImageBitmap(bmp)
        }
    }

    private fun copyToCache(uri: Uri, fallback: String): String? {
        return try {
            val name = queryName(contentResolver, uri) ?: fallback
            val file = File(cacheDir, name)
            contentResolver.openInputStream(uri)?.use { input ->
                file.outputStream().use { output ->
                    input.copyTo(output)
                }
            }
            file.absolutePath
        } catch (e: Exception) {
            null
        }
    }

    private fun queryName(resolver: ContentResolver, uri: Uri): String? {
        val c = resolver.query(uri, null, null, null, null) ?: return null
        c.use {
            if (it.moveToFirst()) {
                val idx = it.getColumnIndex(OpenableColumns.DISPLAY_NAME)
                if (idx >= 0) return it.getString(idx)
            }
        }
        return null
    }

    private fun extractSimilarity(json: String): Double? {
        return try {
            val regex = Regex("\"obtainedSimilarity\"\s*:\s*([0-9]+(?:\\.[0-9]+)?)")
            val m = regex.find(json)
            m?.groupValues?.getOrNull(1)?.toDouble()
        } catch (_: Throwable) { null }
    }

    private fun extractDiffPath(json: String): String? {
        return try {
            val regex = Regex("\"resultImageRef\"\s*:\s*\"([^\"]*)\"")
            regex.find(json)?.groupValues?.getOrNull(1)
        } catch (_: Throwable) { null }
    }

    private fun pretty(json: String): String = try {
        val indent = 2
        val sb = StringBuilder()
        var level = 0
        var inString = false
        for (ch in json) {
            when (ch) {
                '"' -> { sb.append(ch); inString = !inString }
                '{', '[' -> { sb.append(ch); if (!inString) { sb.append('\n'); level++; repeat(level) { sb.append(' '.repeat(indent)) } } }
                '}', ']' -> { if (!inString) { sb.append('\n'); level--; repeat(level) { sb.append(' '.repeat(indent)) } }; sb.append(ch) }
                ',' -> { sb.append(ch); if (!inString) { sb.append('\n'); repeat(level) { sb.append(' '.repeat(indent)) } } }
                else -> sb.append(ch)
            }
        }
        sb.toString()
    } catch (_: Throwable) { json }
}

