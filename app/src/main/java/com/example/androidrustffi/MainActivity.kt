package com.example.androidrustffi

import android.content.Intent
import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity


class MainActivity : AppCompatActivity() {
    private external fun greeting(): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val textView: TextView = findViewById(R.id.txtHello)

    Intent(this, RustService::class.java).also { intent ->
            startService(intent)
        }

        try{
            textView.text = greeting()
        }catch(e:Exception){
            print(e.toString());
        }

    }

    companion object {
        init {
            try{
                System.loadLibrary("rust")
            }catch(e:Exception){
                print(e.toString());
            }
        }
    }
}