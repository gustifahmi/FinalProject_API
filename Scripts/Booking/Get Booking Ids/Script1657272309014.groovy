import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('Booking/Get Booking Ids'))

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, '[0].bookingid', 229)
WS.verifyElementPropertyValue(response, '[1].bookingid', 264)
WS.verifyElementPropertyValue(response, '[2].bookingid', 25)
WS.verifyElementPropertyValue(response, '[3].bookingid', 262)
WS.verifyElementPropertyValue(response, '[4].bookingid', 129)
WS.verifyElementPropertyValue(response, '[5].bookingid', 304)
WS.verifyElementPropertyValue(response, '[6].bookingid', 184)
WS.verifyElementPropertyValue(response, '[7].bookingid', 351)
WS.verifyElementPropertyValue(response, '[8].bookingid', 136)
WS.verifyElementPropertyValue(response, '[9].bookingid', 256)