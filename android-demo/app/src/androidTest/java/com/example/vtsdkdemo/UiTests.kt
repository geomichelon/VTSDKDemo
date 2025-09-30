package com.example.vtsdkdemo

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.ext.junit.rules.ActivityScenarioRule
import androidx.test.espresso.Espresso.onView
import androidx.test.espresso.action.ViewActions.click
import androidx.test.espresso.assertion.ViewAssertions.matches
import androidx.test.espresso.matcher.ViewMatchers.*
import org.hamcrest.CoreMatchers.containsString
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class UiTests {

    @get:Rule
    val rule = ActivityScenarioRule(MainActivity::class.java)

    @Test
    fun compareShowsObtainedSimilarity() {
        // Use sample images to avoid system picker
        onView(withId(R.id.btnPrepare)).perform(click())
        onView(withId(R.id.btnCompare)).perform(click())
        onView(withId(R.id.textJson)).check(matches(withText(containsString("obtainedSimilarity"))))
    }
}

