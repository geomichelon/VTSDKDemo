package com.example.vtsdkdemo

import android.content.ContentResolver
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Bundle
import android.provider.OpenableColumns
import android.widget.*
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import java.io.File

class MainActivity : AppCompatActivity() {
    private lateinit var imgBaseline: ImageView
    private lateinit var imgInput: ImageView
    private lateinit var imgDiff: ImageView
    private lateinit var txtJson: TextView
    private lateinit var txtSim: TextView
    private lateinit var txtAreasCount: TextView
    private lateinit var editX: EditText
    private lateinit var editY: EditText
    private lateinit var editW: EditText
    private lateinit var editH: EditText

    private var baselinePath: String? = null
    private var inputPath: String? = null
    private val excluded = mutableListOf<Rect>()

    data class Rect(val x: Int, val y: Int, val w: Int, val h: Int)

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
        txtAreasCount = findViewById(R.id.textAreasCount)
        editX = findViewById(R.id.editX)
        editY = findViewById(R.id.editY)
        editW = findViewById(R.id.editW)
        editH = findViewById(R.id.editH)

        findViewById<Button>(R.id.btnPickBaseline).setOnClickListener {
            pickBaseline.launch(arrayOf("image/*"))
        }
        findViewById<Button>(R.id.btnPickInput).setOnClickListener {
            pickInput.launch(arrayOf("image/*"))
        }
        findViewById<Button>(R.id.btnPrepare).setOnClickListener { prepareExamples() }
        findViewById<Button>(R.id.btnAddArea).setOnClickListener { addAreaFromInputs() }
        findViewById<Button>(R.id.btnClearAreas).setOnClickListener { excluded.clear(); updateAreasCount() }
        findViewById<Button>(R.id.btnCompare).setOnClickListener { runCompare() }
    }

    private fun runCompare() {
        val b = baselinePath ?: return
        val i = inputPath ?: return
        val exJson = buildExcludedJson()
        val json = VtSdkFFI.vtCompareImages(b, i, 95, 20, exJson, "{\"testName\":\"Android-Demo\"}")
        txtJson.text = pretty(json)
        extractSimilarity(json)?.let { sim -> txtSim.text = String.format("Similarity: %.2f%%", sim) }
        extractDiffPath(json)?.let { path ->
            val bmp = BitmapFactory.decodeFile(path)
            if (bmp != null) imgDiff.setImageBitmap(bmp)
        }
    }

    private fun buildExcludedJson(): String {
        val arr = excluded.map { r ->
            val x1 = r.x.coerceAtLeast(0)
            val y1 = r.y.coerceAtLeast(0)
            val x2 = (r.x + r.w - 1).coerceAtLeast(x1)
            val y2 = (r.y + r.h - 1).coerceAtLeast(y1)
            "{" +
                "\"topLeftX\":$x1,\"topLeftY\":$y1,\"bottomRightX\":$x2,\"bottomRightY\":$y2" +
            "}"
        }
        return "[" + arr.joinToString(",") + "]"
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

    private fun addAreaFromInputs() {
        val x = editX.text.toString().toIntOrNull() ?: 0
        val y = editY.text.toString().toIntOrNull() ?: 0
        val w = editW.text.toString().toIntOrNull() ?: 0
        val h = editH.text.toString().toIntOrNull() ?: 0
        if (w > 0 && h > 0) {
            excluded.add(Rect(x, y, w, h))
            updateAreasCount()
        }
    }

    private fun updateAreasCount() {
        txtAreasCount.text = "Excluded: ${excluded.size}"
    }

    private fun prepareExamples() {
        // baseline: white background with red circle centered, 200x200
        val w = 200
        val h = 200
        val bmpBase = android.graphics.Bitmap.createBitmap(w, h, android.graphics.Bitmap.Config.ARGB_8888)
        val c1 = android.graphics.Canvas(bmpBase)
        c1.drawColor(android.graphics.Color.WHITE)
        val paint = android.graphics.Paint().apply {
            color = android.graphics.Color.RED
            isAntiAlias = true
        }
        c1.drawOval(android.graphics.RectF(50f, 50f, 150f, 150f), paint)

        val bmpIn = android.graphics.Bitmap.createBitmap(w, h, android.graphics.Bitmap.Config.ARGB_8888)
        val c2 = android.graphics.Canvas(bmpIn)
        c2.drawColor(android.graphics.Color.WHITE)
        c2.drawOval(android.graphics.RectF(60f, 50f, 160f, 150f), paint)

        val bFile = File(cacheDir, "baseline.png")
        val iFile = File(cacheDir, "input.png")
        bFile.outputStream().use { out -> bmpBase.compress(android.graphics.Bitmap.CompressFormat.PNG, 100, out) }
        iFile.outputStream().use { out -> bmpIn.compress(android.graphics.Bitmap.CompressFormat.PNG, 100, out) }

        baselinePath = bFile.absolutePath
        inputPath = iFile.absolutePath
        imgBaseline.setImageBitmap(bmpBase)
        imgInput.setImageBitmap(bmpIn)
        imgDiff.setImageDrawable(null)
        txtSim.text = "Similarity: -"
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
