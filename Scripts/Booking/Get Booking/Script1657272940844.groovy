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

response = WS.sendRequest(findTestObject('Booking/Get Booking'))

WS.verifyElementPropertyValue(response, 'firstname', 'Michael')

WS.verifyElementPropertyValue(response, 'lastname', '%date:~4,2%%date:~7,2%%date:~10,4%_%time:~0,2%%time:~3,2%%time:~6,2%')

WS.verifyElementPropertyValue(response, 'totalprice', 111)

WS.verifyElementPropertyValue(response, 'depositpaid', true)

WS.verifyElementPropertyValue(response, 'bookingdates.checkin', '2019-05-16')

WS.verifyElementPropertyValue(response, 'bookingdates.checkout', '2019-05-16')

WS.verifyElementPropertyValue(response, 'additionalneeds', 'Breakfast')

